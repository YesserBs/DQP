# Your first DQP questionnaire

This directory is your workspace; you do not need to modify Rust code.

1. Install the `dqp` command once using the package for your operating system.
2. Open this directory in VS Code.
3. Run:

```bash
dqp run python.dqp.json --output result.json
```

Enter a choice identifier (`a`, `b`, and so on), then optionally enter a justification. The `result.json` report contains your answers, corrections, and the path taken.

The included questionnaire covers Python list indexes. An incorrect first answer triggers a remedial question.

Validate a questionnaire before running it:

```bash
dqp check python.dqp.json
dqp replay python.dqp.json result.json
```

For a new session, choose another report name or omit `--output`; DQP generates a unique name. It refuses to overwrite an existing report. Use `:quit` to stop and save accepted answers.

If `dqp` is not found, finish the installation and reopen the terminal or VS Code. With the portable binary in the package directory, use `./dqp` on Linux/macOS or `.\dqp.exe` on Windows.
