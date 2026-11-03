# One JSON file, one command, one report

## 1. Prepare a workspace

After installing DQP, open the `starter/` directory in VS Code. It contains `python.dqp.json` and a ready-to-use editor configuration.

## 2. Run a questionnaire

Open **Terminal → New Terminal** and run:

```bash
dqp run python.dqp.json --output result.json
```

Enter the choice identifier. A justification is optional: press Enter to skip it. The example sends an incorrect first answer to a remedial question.

## 3. Read the report

`result.json` contains your answers, justifications, corrections, and the path that was followed. The educational result is under `result`; session dates and metadata are under `metadata`.

You can give the report to an AI with:

> Analyze my errors and justifications. Distinguish evidence present in the report from hypotheses. An absent justification is not an error. Suggest three targeted exercises.

## 4. Create another questionnaire

Give your AI the URL of the [standalone DQP guide](v1/AI_GUIDE.md), together with the topic, level, and learning goal. Save its output in a new `.dqp.json` file:

```bash
dqp check new-topic.dqp.json
dqp run new-topic.dqp.json
```

Without `--output`, DQP chooses a different report name automatically. You can copy the `starter` directory for each subject; the engine does not need to be copied.

## Troubleshooting

- **Command not found**: check the installation and PATH, then reopen VS Code.
- **Unknown field or destination**: use the JSON path shown by the diagnostic to fix the file.
- **Report already exists**: choose another output name or omit `--output`.
- **Interrupted session**: `:quit` or Ctrl+C preserves accepted answers. A choice still waiting for its justification is not counted.
