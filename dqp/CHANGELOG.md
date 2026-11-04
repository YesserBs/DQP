# Changelog

## 0.1.2 — reprise des sessions

- Ajout de `dqp continue rapport.json` pour reprendre une session interrompue.
- Les rapports mémorisent le chemin exact du scénario source.
- La reprise vérifie l’empreinte, le parcours et les résultats avant de continuer.

## 0.1.1 — préparation de publication

- README centré sur la création et l’exécution d’un questionnaire.
- Guide DQP 1.0 autonome destiné aux IA, avec exemple complet.
- Dossier starter indépendant du code Rust et schéma VS Code local.
- Packaging d’exécutables avec installateurs locaux et licences.
- Workflow de release en brouillon pour Linux x86-64, Windows x86-64, macOS ARM64 et Intel.
- Contrôle de cohérence du tag, sommes SHA-256 et guide de publication.
- Moteur et sémantique DQP inchangés. Les nouveaux rapports portent engine_version 0.1.1 ; utilisez le moteur 0.1.0 pour rejouer un rapport 0.1.0.

## 0.1.0 — première implémentation

Compilateur strict, moteur déterministe, CLI, rapports, rejeu, spécification, schéma et 41 tests.
