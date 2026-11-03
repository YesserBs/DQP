#!/bin/sh
# Local installer: no network calls, no sudo, no shell configuration edits.
set -eu
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
install_dir="${DQP_INSTALL_DIR:-$HOME/.local/bin}"
if [ ! -f "$script_dir/dqp" ]; then
  echo "Binaire dqp absent. Extrayez le paquet correspondant à votre système." >&2
  exit 1
fi
mkdir -p "$install_dir"
if [ -e "$install_dir/dqp" ]; then
  echo "Installation existante : $install_dir/dqp. Renommez-la avant de la remplacer." >&2
  exit 1
fi
# Validate this executable on the local system before installing.
chmod +x "$script_dir/dqp"
"$script_dir/dqp" --version
install -m 755 "$script_dir/dqp" "$install_dir/dqp"
printf '\nDQP installé dans %s\n' "$install_dir/dqp"
case ":$PATH:" in
  *":$install_dir:"*) printf 'Commande disponible : dqp --version\n' ;;
  *) printf 'Pour ce terminal, ajoutez ce dossier au PATH :\nexport PATH="%s:$PATH"\n' "$install_dir"
     printf 'Pour le conserver, ajoutez la même ligne à la configuration de votre shell.\n' ;;
esac
