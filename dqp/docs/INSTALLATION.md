# Installer DQP

L’installation se fait une fois. Ensuite, ouvrez un dossier contenant votre questionnaire et lancez `dqp run questionnaire.dqp.json`.

## Choisir le paquet

Dans la section **Releases** du dépôt :

| Ordinateur | Suffixe du paquet |
|---|---|
| Linux Intel/AMD 64 bits | `linux-x86_64.tar.gz` |
| Windows Intel/AMD 64 bits | `windows-x86_64.zip` |
| Mac Apple Silicon (M1, M2…) | `macos-arm64.tar.gz` |
| Mac Intel | `macos-x86_64.tar.gz` |

Les paquets Linux de la CI sont construits sur Ubuntu 22.04. Ils ciblent Linux GNU/glibc ; ils ne sont pas destinés à Alpine/musl ou aux processeurs ARM. Le paquet local livré avant la première release a été construit sur Ubuntu 24.04 : il peut demander une glibc plus récente. Les paquets macOS sont construits sur macOS 15 et ne sont pas notarisés. Windows et macOS peuvent demander une autorisation d’exécution pour un programme téléchargé ; utilisez les mécanismes locaux de vérification et ne désactivez pas globalement les protections du système.

Le fichier `SHA256SUMS.txt` accompagne les releases pour vérifier les téléchargements. Il ne constitue pas une signature de l’auteur.

## Linux et macOS

Extrayez le paquet, puis ouvrez un terminal dans le dossier extrait :

```bash
./dqp --version
sh install.sh
```

Le programme est copié dans `~/.local/bin` sans privilèges administrateur. L’installateur affiche la ligne à utiliser si ce dossier n’est pas dans le PATH. Pour l’installation standard, dans le terminal courant :

```bash
export PATH="$HOME/.local/bin:$PATH"
dqp --version
```

Pour conserver ce réglage, ajoutez la ligne `export PATH=...` à `~/.bashrc` pour Bash ou `~/.zshrc` pour Zsh, puis rouvrez le terminal et VS Code. Le script n’édite pas ces fichiers à votre place et refuse d’écraser une installation existante.

Ouvrez ensuite le dossier `starter` dans VS Code :

```bash
dqp run python.dqp.json --output resultat.json
```

Sans installation, le binaire reste portable. Depuis le dossier du paquet :

```bash
./dqp run starter/python.dqp.json --output resultat.json
```

## Windows

Extrayez l’archive ZIP. Dans PowerShell, depuis son dossier :

```powershell
.\dqp.exe --version
.\install.ps1
```

Le script copie le programme dans `%LOCALAPPDATA%\DQP\bin`. Si la politique locale empêche l’exécution du script, vous pouvez créer ce dossier et y copier `dqp.exe` manuellement. Aucun changement global de politique PowerShell n’est nécessaire.

Pour le terminal actuel :

```powershell
$env:Path = "$env:LOCALAPPDATA\DQP\bin;" + $env:Path
dqp --version
```

Pour conserver ce réglage, ajoutez `%LOCALAPPDATA%\DQP\bin` au **Path utilisateur** dans les variables d’environnement Windows, puis rouvrez VS Code.

Vous pouvez aussi utiliser directement :

```powershell
.\dqp.exe run starter\python.dqp.json --output resultat.json
```

## Installation depuis les sources

Pour les développeurs, ou avant la première release : installez Rust/Cargo, ouvrez un terminal dans le dépôt, puis :

```bash
cargo install --path . --locked
dqp --version
```

Si Cargo vient d’être installé sous Linux/macOS : `source "$HOME/.cargo/env"`. Sous Ubuntu, `sudo apt install build-essential` fournit les outils de compilation système si nécessaire.

## Mise à jour et désinstallation

Les installateurs des paquets n’écrasent pas une version existante. Renommez le binaire précédent avant d’installer la nouvelle version ; conservez-le si vous devez rejouer des rapports produits avec cette version. Les rapports sont liés à la version du moteur : une migration automatique n’est pas fournie.

Pour désinstaller un paquet portable, supprimez le binaire installé et, si vous l’aviez ajoutée exclusivement pour DQP, l’entrée PATH correspondante. Une installation faite avec Cargo se supprime avec `cargo uninstall dqp`.
