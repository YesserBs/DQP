# Un JSON, une commande, un rapport

## 1. Préparer le dossier

Après l’installation de DQP, ouvrez `starter/` dans VS Code. Vous avez un fichier `python.dqp.json` et un mode d’emploi. La configuration d’édition est déjà incluse.

## 2. Lancer

Dans **Terminal → Nouveau terminal** :

```bash
dqp run python.dqp.json --output resultat.json
```

Saisissez l’identifiant du choix. La justification est toujours facultative : Entrée suffit pour passer. Dans l’exemple, une première erreur dirige vers une question de rattrapage.

## 3. Lire le résultat

`resultat.json` contient vos réponses, vos justifications, les corrections et le chemin suivi. Le résultat pédagogique se trouve dans `result` ; les dates et informations de session dans `metadata`.

Vous pouvez transmettre ce fichier à une IA avec :

> Analyse mes erreurs et mes justifications. Distingue les preuves présentes dans le rapport et les hypothèses. Une justification absente n’est pas une erreur. Propose trois exercices ciblés.

## 4. Créer un autre questionnaire

Fournissez à votre IA l’URL de [la documentation autonome](v1/AI_GUIDE.md), avec la notion, le niveau et l’objectif. Enregistrez sa sortie dans un nouveau fichier `.dqp.json`.

```bash
dqp check nouveau.dqp.json
dqp run nouveau.dqp.json
```

Sans `--output`, DQP choisit un nom de rapport différent. Vous pouvez copier le dossier starter pour chaque matière : le moteur est déjà installé et n’a pas à être recopié.

## En cas d’erreur

- **Commande introuvable** : vérifiez l’installation et le PATH, puis rouvrez VS Code.
- **Champ ou destination inconnus** : le diagnostic indique le chemin JSON et l’emplacement à corriger ; vous pouvez transmettre ce diagnostic à l’IA.
- **Rapport déjà existant** : choisissez un nouveau nom ou omettez `--output`.
- **Interruption** : `:quit` ou Ctrl+C conservent les réponses déjà acceptées. Un choix encore en attente de justification ne compte pas.
