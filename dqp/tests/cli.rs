use serde_json::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dqp"))
}
fn source() -> String {
    format!("{}/examples/branching.dqp.json", env!("CARGO_MANIFEST_DIR"))
}
#[test]
fn cli_compile_run_replay() {
    let dir = tempfile::tempdir().unwrap();
    let compiled = dir.path().join("test.dqpc");
    let report = dir.path().join("session.json");
    assert!(cli()
        .args(["compile", &source(), "--out", compiled.to_str().unwrap()])
        .output()
        .unwrap()
        .status
        .success());
    let mut child = cli()
        .args([
            "run",
            compiled.to_str().unwrap(),
            "--output",
            report.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"z\na\nreason\na\n\nb\n\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let r: Value = serde_json::from_slice(&std::fs::read(&report).unwrap()).unwrap();
    assert_eq!(r["result"]["counters"]["answered_count"], 3);
    assert_eq!(r["result"]["steps"][0]["input"]["justification"], "reason");
    assert_eq!(
        r["metadata"]["feedback_written"].as_array().unwrap().len(),
        3
    );
    let replay = cli()
        .args([
            "replay",
            compiled.to_str().unwrap(),
            report.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        replay.status.success(),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
}
#[test]
fn refuses_existing_output() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("important.json");
    std::fs::write(&path, "keep").unwrap();
    let output = cli()
        .args(["run", &source(), "--output", path.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(std::fs::read_to_string(path).unwrap(), "keep");
}
#[test]
fn invalid_compile_produces_no_artifact() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("bad.dqpc");
    let input = format!(
        "{}/examples/invalid-target.dqp.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let r = cli()
        .args([
            "compile",
            &input,
            "--out",
            out.to_str().unwrap(),
            "--diagnostics",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(r.status.code(), Some(2));
    assert!(!out.exists());
    let errors: Value = serde_json::from_slice(&r.stderr).unwrap();
    assert_eq!(errors[0]["code"], "DQP-E205");
}
#[test]
fn eof_preserves_partial_report_and_replay() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("partial.json");
    let mut child = cli()
        .args(["run", &source(), "--output", out.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"a\n\n").unwrap();
    let result = child.wait_with_output().unwrap();
    assert_eq!(result.status.code(), Some(130));
    let r: Value = serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
    assert_eq!(r["result"]["status"], "interrupted");
    assert_eq!(r["result"]["steps"].as_array().unwrap().len(), 1);
    assert!(cli()
        .args(["replay", &source(), out.to_str().unwrap()])
        .output()
        .unwrap()
        .status
        .success());
}
#[test]
fn quit_before_justification_does_not_commit_choice() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("partial.json");
    let mut child = cli()
        .args(["run", &source(), "--output", out.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"a\n:quit\n")
        .unwrap();
    assert_eq!(child.wait().unwrap().code(), Some(130));
    let r: Value = serde_json::from_slice(&std::fs::read(out).unwrap()).unwrap();
    assert_eq!(r["result"]["counters"]["answered_count"], 0);
}

#[test]
fn continue_resumes_interrupted_session() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("partial.json");
    let mut child = cli()
        .args(["run", &source(), "--output", out.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"a\n\n").unwrap();
    assert_eq!(child.wait().unwrap().code(), Some(130));

    let mut resumed = cli()
        .args(["continue", out.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    resumed
        .stdin
        .take()
        .unwrap()
        .write_all(b"a\n\nb\n\n")
        .unwrap();
    let resumed = resumed.wait_with_output().unwrap();
    assert!(
        resumed.status.success(),
        "{}",
        String::from_utf8_lossy(&resumed.stderr)
    );
    let report: Value = serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
    assert_eq!(report["result"]["status"], "completed");
    assert_eq!(report["result"]["steps"].as_array().unwrap().len(), 3);
    assert_eq!(report["metadata"]["source_path"], source());
}
