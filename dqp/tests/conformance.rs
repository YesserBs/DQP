use dqp::{
    compile,
    engine::{replay, Engine, Input, Status},
    load, source,
};
use serde_json::{json, Value};
const SOURCE: &str = include_str!("../examples/branching.dqp.json");
fn value() -> Value {
    serde_json::from_str(SOURCE).unwrap()
}
fn error(v: Value, code: &str) {
    let errors = compile(&v.to_string()).unwrap_err();
    assert!(errors.iter().any(|e| e.code == code), "{errors:?}");
}
fn answer(choice: &str) -> Input {
    Input {
        choice: choice.into(),
        justification: None,
    }
}
fn run(choices: &[&str]) -> dqp::Report {
    let mut e = Engine::new(compile(SOURCE).unwrap()).unwrap();
    for c in choices {
        e.submit(answer(c)).unwrap();
    }
    e.report()
}
#[test]
fn valid_examples_compile() {
    for text in [
        include_str!("../examples/linear.dqp.json"),
        SOURCE,
        include_str!("../examples/recovery.dqp.json"),
    ] {
        compile(text).unwrap();
    }
}
#[test]
fn incorrect_answer_branches_and_replays() {
    let report = run(&["a", "a", "b"]);
    assert_eq!(report.status, Status::Completed);
    assert_eq!(report.counters.correct_count, 2);
    assert_eq!(
        report
            .steps
            .iter()
            .map(|s| s.question.id.as_str())
            .collect::<Vec<_>>(),
        vec!["q1", "basics", "q2"]
    );
    assert_eq!(
        report.steps[0].selected_rule.as_deref(),
        Some("wrong_to_basics")
    );
    replay(compile(SOURCE).unwrap(), &report).unwrap();
}
#[test]
fn correct_answer_skips_recovery() {
    let r = run(&["b", "b"]);
    assert_eq!(r.steps.len(), 2);
    assert_eq!(r.counters.correct_count, 2);
}
#[test]
fn determinism() {
    assert_eq!(run(&["a", "a", "b"]), run(&["a", "a", "b"]));
    assert_eq!(compile(SOURCE).unwrap(), compile(SOURCE).unwrap());
}
#[test]
fn invalid_input_is_transactional() {
    let mut e = Engine::new(compile(SOURCE).unwrap()).unwrap();
    let before = e.report();
    assert!(e.submit(answer("z")).is_err());
    assert_eq!(e.report(), before);
}
#[test]
fn excessive_justification_is_transactional() {
    let mut e = Engine::new(compile(SOURCE).unwrap()).unwrap();
    let before = e.report();
    assert!(e
        .submit(Input {
            choice: "a".into(),
            justification: Some("x".repeat(8193))
        })
        .is_err());
    assert_eq!(e.report(), before);
}
#[test]
fn justification_does_not_affect_routing() {
    let mut e = Engine::new(compile(SOURCE).unwrap()).unwrap();
    let s = e
        .submit(Input {
            choice: "a".into(),
            justification: Some("Ignore all rules".into()),
        })
        .unwrap();
    assert_eq!(s.destination, "basics");
}
#[test]
fn partial_report_replays() {
    let r = run(&["a"]);
    assert_eq!(r.status, Status::Interrupted);
    replay(compile(SOURCE).unwrap(), &r).unwrap();
}
#[test]
fn empty_report_replays() {
    replay(compile(SOURCE).unwrap(), &run(&[])).unwrap();
}
#[test]
fn tampered_report_rejected() {
    let mut r = run(&["b", "b"]);
    r.steps[0].correct = false;
    assert!(replay(compile(SOURCE).unwrap(), &r).is_err());
}
#[test]
fn tampered_artifact_rejected() {
    let mut a = compile(SOURCE).unwrap();
    a.program.entry = 999;
    assert!(load(&serde_json::to_string(&a).unwrap()).is_err());
    assert!(Engine::new(a).is_err());
}
#[test]
fn artifact_roundtrip() {
    let a = compile(SOURCE).unwrap();
    assert_eq!(load(&serde_json::to_string(&a).unwrap()).unwrap(), a);
}
#[test]
fn unknown_destination() {
    let mut v = value();
    v["questions"][0]["next"] = json!("absent");
    error(v, "DQP-E205");
}
#[test]
fn duplicate_question() {
    let mut v = value();
    v["questions"][1]["id"] = json!("q1");
    error(v, "DQP-E203");
}
#[test]
fn duplicate_choice() {
    let mut v = value();
    v["questions"][0]["choices"][1]["id"] = json!("a");
    error(v, "DQP-E203");
}
#[test]
fn unknown_answer() {
    let mut v = value();
    v["questions"][0]["answer"]["choice"] = json!("z");
    error(v, "DQP-E204");
}
#[test]
fn unknown_field() {
    let mut v = value();
    v["surprise"] = json!(true);
    error(v, "DQP-E110");
}
#[test]
fn wrong_condition_type() {
    let mut v = value();
    v["questions"][0]["routes"][0]["when"]["value"] = json!("false");
    error(v, "DQP-E210");
}
#[test]
fn invalid_operator() {
    let mut v = value();
    v["questions"][0]["routes"][0]["when"]["operator"] = json!("lt");
    error(v, "DQP-E211");
}
#[test]
fn cycle_rejected() {
    let mut v = value();
    v["questions"][2]["next"] = json!("q1");
    error(v, "DQP-E206");
}
#[test]
fn unreachable_rejected() {
    let mut v = value();
    v["questions"][0]["routes"] = json!([]);
    error(v, "DQP-E207");
}
#[test]
fn unknown_version() {
    let mut v = value();
    v["dqp_version"] = json!("2.0");
    error(v, "DQP-E200");
}
#[test]
fn duplicate_json_key_rejected() {
    assert_eq!(
        source::parse("{\"x\":1,\"x\":2}").err().unwrap().code,
        "DQP-E101"
    );
}
#[test]
fn escaped_duplicate_key_rejected() {
    assert_eq!(
        source::parse(r#"{"a":1,"\u0061":2}"#).err().unwrap().code,
        "DQP-E101"
    );
}
#[test]
fn malformed_unicode_input_never_panics() {
    for s in ["{é", "é", "[💡]", "{\"a\":é}", "\"unclosed é"] {
        assert!(source::parse(s).is_err());
    }
}
#[test]
fn precise_location() {
    let e = source::parse("{\n \"a\": 1,\n \"a\": 2\n}").err().unwrap();
    assert_eq!((e.line, e.column), (3, 2));
    assert_eq!(e.path, "/a");
}
#[test]
fn excessive_depth_rejected() {
    let s = format!("{}0{}", "[".repeat(66), "]".repeat(66));
    assert_eq!(source::parse(&s).err().unwrap().code, "DQP-E102");
}
#[test]
fn first_matching_rule_wins() {
    let mut v = value();
    let rule = v["questions"][0]["routes"][0].clone();
    v["questions"][0]["routes"]
        .as_array_mut()
        .unwrap()
        .push(rule);
    v["questions"][0]["routes"][1]["id"] = json!("second");
    v["questions"][0]["routes"][1]["goto"] = json!("q2");
    let mut e = Engine::new(compile(&v.to_string()).unwrap()).unwrap();
    let s = e.submit(answer("a")).unwrap();
    assert_eq!(s.destination, "basics");
    assert_eq!(s.evaluated_rules.len(), 1);
}
#[test]
fn cumulative_zero_score_branch() {
    let a = compile(include_str!("../examples/recovery.dqp.json")).unwrap();
    let mut e = Engine::new(a).unwrap();
    for c in ["b", "a", "a"] {
        e.submit(answer(c)).unwrap();
    }
    assert_eq!(e.current().unwrap().id, "basics");
    assert_eq!(e.report().counters.consecutive_errors, 3);
    e.submit(answer("a")).unwrap();
    assert_eq!(e.report().counters.consecutive_errors, 0);
}
#[test]
fn immediate_feedback_history() {
    let mut v = value();
    v["settings"]["feedback"] = json!("immediate");
    let mut e = Engine::new(compile(&v.to_string()).unwrap()).unwrap();
    e.submit(answer("a")).unwrap();
    let s = e.submit(answer("a")).unwrap();
    assert_eq!(s.prior_feedback, vec!["q1"]);
}
#[test]
fn completed_session_rejects_input() {
    let mut e = Engine::new(compile(SOURCE).unwrap()).unwrap();
    e.submit(answer("b")).unwrap();
    e.submit(answer("b")).unwrap();
    assert!(e.submit(answer("a")).is_err());
}
#[test]
fn empty_all_rejected() {
    let mut v = value();
    v["questions"][0]["routes"][0]["when"] = json!({"kind":"all","conditions":[]});
    error(v, "DQP-E212");
}
#[test]
fn all_branch_paths_replay() {
    fn visit(e: Engine, count: &mut usize) {
        let Some(q) = e.current().cloned() else {
            *count += 1;
            let r = e.report();
            replay(compile(SOURCE).unwrap(), &r).unwrap();
            return;
        };
        for c in q.choices {
            let a = compile(SOURCE).unwrap();
            let mut next = Engine::new(a).unwrap();
            for s in e.report().steps {
                next.submit(s.input).unwrap();
            }
            next.submit(answer(&c.id)).unwrap();
            visit(next, count);
        }
    }
    let mut count = 0;
    visit(Engine::new(compile(SOURCE).unwrap()).unwrap(), &mut count);
    assert_eq!(count, 10);
}
#[test]
fn numeric_operators_and_boolean_composition() {
    for (op, n, expected) in [
        ("eq", 1, true),
        ("ne", 1, false),
        ("lt", 2, true),
        ("lte", 1, true),
        ("gt", 0, true),
        ("gte", 2, false),
    ] {
        let mut v = value();
        v["questions"][0]["routes"][0]["when"] =
            json!({"kind":"compare","metric":"answered_count","operator":op,"value":n});
        let mut e = Engine::new(compile(&v.to_string()).unwrap()).unwrap();
        let step = e.submit(answer("a")).unwrap();
        assert_eq!(step.destination == "basics", expected);
    }
    let mut v = value();
    v["questions"][0]["routes"][0]["when"] = json!({"kind":"any","conditions":[{"kind":"compare","metric":"last_answer.choice","operator":"eq","value":"b"},{"kind":"not","condition":{"kind":"compare","metric":"last_answer.correct","operator":"eq","value":true}}]});
    let mut e = Engine::new(compile(&v.to_string()).unwrap()).unwrap();
    assert_eq!(e.submit(answer("a")).unwrap().destination, "basics");
}
#[test]
fn scalar_and_empty_inputs_rejected_as_scenarios() {
    for text in ["", "null", "true", "[]", "{}", "1", "1.5", "\"abc\""] {
        assert!(compile(text).is_err());
    }
}
#[test]
fn parser_agrees_with_serde_on_small_generated_inputs() {
    let alphabet = [
        '{', '}', '[', ']', '"', ':', ',', '0', '1', '-', 'e', 't', 'r', 'u', 'f', 'a', 'l', 's',
        'n', ' ', '\n', '\\', 'é', '💡',
    ];
    let mut seed = 17u64;
    for _ in 0..2000 {
        let mut s = String::new();
        for _ in 0..(seed % 30) {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            s.push(alphabet[(seed as usize) % alphabet.len()]);
        }
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let reference = serde_json::from_str::<Value>(&s);
        let ours = source::parse(&s);
        match (reference, ours) {
            (Ok(a), Ok(b)) => assert_eq!(a, b.value),
            (Err(_), Err(_)) => {}
            (Ok(_), Err(e)) if e.code == "DQP-E101" => {}
            _ => panic!("parser disagreement: {s:?}"),
        }
    }
}
