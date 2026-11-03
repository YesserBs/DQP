# Vérification de la livraison 0.1.0

Vérification locale effectuée le 2 novembre 2026 sur Linux x86-64 avec Rust/Cargo 1.75.0. Les dépendances exactes sont consignées dans Cargo.lock.

## Résultats

| Vérification | Résultat |
|---|---|
| `cargo test --locked` | 41 tests réussis, 0 échec |
| Tests de conformité | 36 réussis |
| Tests d’intégration CLI | 5 réussis |
| `cargo fmt --all -- --check` | réussi |
| `cargo build --release --locked` | réussi |
| Exécution et rejeu de l’exemple branching | réussi, chemin q1 → basics → q2, score 2/3 |
| Vérification et rejeu avec le binaire release | réussi |
| Signal SIGINT sur une session ouverte | code 130 et rapport partiel valide |
| Validité du JSON Schema 2020-12 | vérifiée avec jsonschema |
| Trois exemples valides soumis au JSON Schema | acceptés |

La suite couvre notamment : clés dupliquées et échappées, destinations et réponses inconnues, cycles, questions inaccessibles, types et opérateurs, priorité des règles, composition booléenne, compteurs cumulés, exactitude des positions, JSON Unicode invalide, limites de profondeur, justification trop longue, invariance de l’état après saisie invalide, artefact modifié, rapport modifié, sessions partielles et terminées.

Un test compare le lecteur DQP à serde_json sur 2 000 petites entrées générées de manière déterministe. Un autre explore les 10 chemins de réponses du scénario branching et vérifie leur rejeu. Cela ne constitue pas une preuve formelle ou un fuzzing exhaustif.

Le fichier `examples/session.json` a été produit par l’exécutable à partir de `examples/branching.answers.txt` : ce n’est pas un rapport rédigé manuellement.

## Portée

Les tests ont été exécutés localement sur Linux uniquement. Le workflow fourni prépare aussi la vérification macOS et Windows, mais ces exécutions distantes n’ont pas été réalisées dans cette livraison. Le projet n’a pas fait l’objet d’un audit externe, de benchmarks ou d’une vérification d’interopérabilité QTI/xAPI.

L’archive contient les sources, les dépendances verrouillées, les exemples et la documentation. Elle n’embarque pas de binaire précompilé ni les dépendances téléchargées. `cargo build --release --locked` produit le binaire adapté à la plateforme locale.
