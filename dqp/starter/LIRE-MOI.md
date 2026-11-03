# Mon premier questionnaire DQP

Ce dossier est votre espace de travail : aucun code Rust à modifier.

1. Installez une fois la commande `dqp` depuis le paquet adapté à votre système.
2. Ouvrez ce dossier dans VS Code.
3. Dans le terminal intégré, lancez :

```bash
dqp run python.dqp.json --output resultat.json
```

Répondez avec l’identifiant du choix (`a`, `b`…), puis une justification ou Entrée pour passer. Le rapport `resultat.json` apparaît dans ce dossier. Vous pouvez le donner à une IA pour discuter de vos réponses.

Le questionnaire fourni porte sur les indices des listes Python. En cas d’erreur à la première question, une question de rattrapage est posée.

Vous pouvez remplacer `python.dqp.json` par un questionnaire généré selon la documentation DQP 1.0. Gardez l’extension `.dqp.json` pour l’aide à la saisie dans VS Code. Le schéma fourni avec le dossier fonctionne sans accès réseau, mais seul `dqp check` vérifie toutes les règles du protocole.

```bash
# Vérifier avant de lancer
dqp check python.dqp.json

# Vérifier le chemin enregistré
dqp replay python.dqp.json resultat.json
```

Pour une nouvelle session, choisissez un autre nom de rapport ou omettez `--output` : DQP créera un nom automatiquement. Il refuse d’écraser un résultat existant. `:quit` interrompt la session et sauvegarde les réponses déjà acceptées.

Si « commande introuvable » apparaît : terminez l’installation du paquet DQP et rouvrez le terminal ou VS Code. Avec le binaire portable placé dans ce dossier, utilisez `./dqp` sous Linux/macOS ou `.\dqp.exe` sous Windows à la place de `dqp`.
