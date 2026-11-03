# Installing DQP

Install DQP once. After that, open a directory containing a questionnaire and run:

```bash
dqp run questionnaire.dqp.json
```

## Prebuilt packages

Download the package matching your computer from the repository's **Releases** page:

| Computer | Package suffix |
|---|---|
| Linux Intel/AMD 64-bit | `linux-x86_64.tar.gz` |
| Windows Intel/AMD 64-bit | `windows-x86_64.zip` |
| macOS Apple Silicon | `macos-arm64.tar.gz` |
| macOS Intel | `macos-x86_64.tar.gz` |

### Linux and macOS

Extract the archive, open a terminal in the extracted directory, and run:

```bash
./dqp --version
sh install.sh
```

The installer copies the executable to `~/.local/bin` without administrator privileges. If that directory is not already in your PATH, add it to the current shell:

```bash
export PATH="$HOME/.local/bin:$PATH"
dqp --version
```

To keep the setting permanently, add the same `export PATH=...` line to `~/.bashrc` (Bash) or `~/.zshrc` (Zsh), then reopen the terminal and VS Code. You can now run `dqp ...` from any directory.

Without installation, run the portable executable directly:

```bash
./dqp run starter/python.dqp.json --output result.json
```

### Windows

Extract the ZIP archive and run PowerShell from the extracted directory:

```powershell
.\dqp.exe --version
.\install.ps1
```

The installer copies the executable to `%LOCALAPPDATA%\DQP\bin`. For the current PowerShell session:

```powershell
$env:Path = "$env:LOCALAPPDATA\DQP\bin;" + $env:Path
dqp --version
```

To keep it permanently, add `%LOCALAPPDATA%\DQP\bin` to the user **Path** environment variable, then reopen VS Code. You can now run `dqp ...` from any directory.

## Install from source

Install Rust and Cargo, open a terminal at the repository root (`dqp/`), and run:

```bash
cargo test --locked
cargo install --path . --locked
dqp --version
```

Cargo installs the command in `~/.cargo/bin`. Ensure that directory is in your PATH. On Ubuntu, install `build-essential` if the linker tools are missing.

## Updating and uninstalling

Package installers do not overwrite an existing installation. Rename the previous executable before installing a new version if you need to keep it for replaying reports. To remove a Cargo installation:

```bash
cargo uninstall dqp
```
