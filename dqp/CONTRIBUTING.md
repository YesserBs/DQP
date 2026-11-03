# Contribuer à DQP

## Développer

```bash
cargo test --locked
cargo fmt --all -- --check
cargo run --locked -- run examples/branching.dqp.json
```

Les modules Rust sont documentés dans `docs/ARCHITECTURE.md`. Les scripts de distribution utilisent uniquement la bibliothèque standard Python 3.9+. Le packaging appelle aussi Cargo pour récupérer les licences des dépendances verrouillées.

## Proposer un changement

Décrivez le problème, le comportement attendu et un petit scénario reproductible. Pour une correction, ajoutez un test qui échouait avant la modification. Pour une évolution de format, mettez à jour la spécification, le JSON Schema, le guide IA et les exemples. Ne changez pas silencieusement la signification d’une règle déjà publiée.

## Versions

- Le numéro du logiciel suit `Cargo.toml` et `Cargo.lock`.
- La version source DQP est `dqp_version`.
- L’artefact compilé et le rapport ont leurs versions de format.
- Le rapport conserve aussi la version logicielle du moteur ; le rejeu strict exige la même version.

Une correction de documentation/distribution peut incrémenter le patch logiciel sans changer le langage. Les versions 0.x restent expérimentales. La licence MIT couvre les contributions ; ne soumettez que du contenu que vous avez le droit de proposer sous cette licence.
