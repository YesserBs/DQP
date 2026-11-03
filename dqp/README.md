# DQP · Diverging Questions Protocol

**Un questionnaire qui s’adapte à vos réponses, avec un parcours explicable et un rapport à analyser.**

Écrivez ou faites générer un petit fichier JSON, lancez une commande, puis récupérez vos réponses et vos raisonnements dans un rapport JSON.

```bash
dqp run python.dqp.json --output resultat.json
```

**Statut : version expérimentale 0.1.1 · format DQP 1.0 · licence MIT.** Les noms de versions désignent ce projet, sans certification par un organisme externe.

## Je veux réviser

1. Ouvrez la section **Releases** du dépôt et téléchargez le paquet adapté à votre ordinateur. Si aucune release n’est encore publiée, utilisez l’installation depuis les sources décrite ci-dessous.
2. Extrayez le paquet et suivez son fichier `INSTALLATION.md`. Avec un paquet précompilé, Rust et Cargo ne sont pas nécessaires.
3. Ouvrez le dossier `starter` dans VS Code, puis lancez la commande ci-dessus.

Répondez avec `a`, `b` ou `c`. À chaque question, vous pouvez expliquer votre raisonnement ou simplement appuyer sur Entrée. Le fichier `resultat.json` contient les réponses, les corrections et les raisons du chemin suivi.

Pour recommencer sans choisir un nom de sortie, lancez `dqp run python.dqp.json` : un nouveau rapport sera nommé automatiquement. DQP refuse d’écraser un rapport existant.

[Installation](docs/INSTALLATION.md) · [Premier questionnaire](docs/QUICKSTART.md) · [Guide pour les IA](docs/v1/AI_GUIDE.md)

## Demander un questionnaire à une IA

Ouvrez le [guide autonome DQP 1.0](docs/v1/AI_GUIDE.md), copiez son URL publique et demandez :

> Lis cette documentation : [URL du guide]. Génère uniquement un scénario DQP 1.0 pour réviser les listes Python, niveau débutant, avec un rattrapage sur les indices. Respecte exactement le format décrit. Si tu ne peux pas accéder au guide, indique-le avant de générer le fichier.

Enregistrez le résultat dans `python.dqp.json`, puis vérifiez-le :

```bash
dqp check python.dqp.json
```

La documentation publiée peut être consultée par une IA disposant d’un accès web. Elle n’est pas automatiquement connue de tous les modèles. Le compilateur vérifie la conformité technique ; relisez les corrections pour leur exactitude pédagogique.

## Pourquoi DQP ?

- **Adaptation explicite** : une erreur peut déclencher un exercice de rattrapage ; plusieurs échecs peuvent orienter vers les bases.
- **Exécution déterministe** : les mêmes entrées sur le même programme et la même version du moteur donnent les mêmes résultats.
- **Raisonnement facultatif** : les justifications sont conservées, sans être notées automatiquement.
- **Rapport autonome** : questions, choix, corrections, compteurs et règles de transition sont enregistrés.
- **Fonctionnement local** : pas de compte, clé API ou connexion IA imposée.

## Une commande pour chaque besoin

| Commande | Utilité |
|---|---|
| `dqp check python.dqp.json` | Vérifier le scénario sans le lancer |
| `dqp run python.dqp.json` | Répondre et créer un rapport |
| `dqp compile python.dqp.json --out python.dqpc` | Produire un programme compilé |
| `dqp replay python.dqp.json resultat.json` | Vérifier les décisions du rapport |

Le dossier utilisateur ne contient pas `src/` ou `Cargo.toml`. Ceux-ci appartiennent au dépôt destiné aux contributeurs.

## Je veux développer DQP

Installez Rust et Cargo depuis [rust-lang.org](https://rust-lang.org/tools/install/), puis depuis ce dépôt :

```bash
cargo test --locked
cargo install --path . --locked
```

Assurez-vous que `~/.cargo/bin` est dans votre PATH. Vous pouvez ensuite utiliser `dqp` dans n’importe quel dossier, notamment `starter/`. Sous Ubuntu, la compilation nécessite aussi les outils d’édition de liens usuels, disponibles dans `build-essential`.

[Contribuer](CONTRIBUTING.md) · [Architecture](docs/ARCHITECTURE.md) · [Spécification](docs/SPECIFICATION.md) · [Validation](docs/VALIDATION.md) · [Publier une release](docs/PUBLISHING.md)

## Périmètre actuel

QCM à réponse unique, parcours sans boucle, compteurs entiers, corrections immédiates ou finales, justifications libres sur une ligne. Les traces restent accessibles sans IA. Le moteur ne vérifie pas la vérité des corrections et ne mesure pas une maîtrise pédagogique certifiée. Il n’offre pas de dispositif anti-triche ou de signature des rapports.

Les versions précompilées sont produites par le workflow de release. Consultez le statut de ce workflow et les assets réellement publiés ; la présence du fichier de workflow n’atteste pas à elle seule une compatibilité plateforme.
