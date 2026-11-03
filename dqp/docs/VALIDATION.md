# Validation de la préparation de publication 0.1.1

Vérifications locales terminées le 3 novembre 2026 sur Linux x86-64. La version 0.1.1 modifie la distribution et la documentation ; les fichiers Rust du moteur sont inchangés par rapport à 0.1.0. La version logicielle intégrée au rapport change naturellement.

| Vérification | Résultat |
|---|---|
| Suite Rust `cargo test --locked` | 41 tests réussis, aucun échec |
| Compilation optimisée 0.1.1 | réussie |
| Formatage Rust | conforme |
| Tests Python de distribution | 6 réussis |
| Scénario starter et rejeu | réussis |
| Exemple complet extrait du guide IA | accepté par le compilateur |
| Schéma VS Code du starter | identique au schéma de référence |
| Contrôle du tag de version | bon tag accepté, autre version refusée |
| Génération d’une URL de documentation épinglée | vérifiée sur une URL de dépôt de test |
| Installateur Unix | installation dans un dossier temporaire réussie ; refus d’écrasement vérifié |
| Syntaxe shell de l’installateur | vérifiée |
| Syntaxe YAML des workflows | analysée avec succès |

La validation YAML est syntaxique, pas une exécution de GitHub Actions. Les quatre jobs de release devront être exécutés sur le dépôt réel. Windows, macOS et l’installateur PowerShell n’ont pas été exécutés localement.

Le binaire Linux local est construit dans l’environnement Ubuntu 24.04 de préparation ; il peut demander une glibc plus récente que celui qui sera construit sur Ubuntu 22.04 par la CI. Ce paquet local n’est pas une release GitHub publiée. Les paquets macOS ne sont pas notarisés et aucune signature d’éditeur n’est fournie.

La version 0.1.0 et ses résultats de vérification sont conservés dans `VALIDATION_0.1.0.md`. Pour les rapports de cette version, utilisez son moteur : le rejeu strict ne migre pas les rapports entre versions logicielles.

## Vérification des archives finales

Le paquet Linux a été extrait dans un dossier temporaire et son propre exécutable a lancé le questionnaire starter : rapport terminé, score attendu 2/2. Le paquet ne contient aucun `src/`. Les licences sont présentes, l’archive starter comprend quatre fichiers dont la configuration VS Code, et les sommes SHA-256 correspondent aux archives livrées.
