# DQP 0.1.1 — kit de publication

## Pour le dépôt GitHub

Le dossier **dqp/** contient le projet à pousser, avec les sources, les tests, les workflows et les documents. Sa racine contient Cargo.toml et README.md.

Si vous avez déjà un dépôt, intégrez ces fichiers en conservant votre `.git` et vos modifications personnelles, puis examinez le diff. La procédure est dans **dqp/docs/PUBLISHING.md**.

Après le push du code et le succès de la CI, le tag `v0.1.1` déclenche les constructions Linux, Windows et macOS. Le workflow crée une release en brouillon à vérifier avant publication. Rien n’a été publié à distance dans cette livraison.

## Pour tester sans Rust sur Linux

Dans **packages/**, extrayez `dqp-v0.1.1-linux-x86_64.tar.gz`, ouvrez un terminal dans son dossier, puis :

```bash
./dqp run starter/python.dqp.json --output resultat.json
```

Ce paquet est construit et testé localement sur Ubuntu 24.04 x86-64. Les autres plateformes seront construites par GitHub Actions. Pour installer la commande de façon réutilisable, suivez INSTALLATION.md dans le paquet.

## Pour les futurs utilisateurs

Distribuez les paquets de la release, pas le dépôt de développement. Chaque paquet comprend un exécutable, un installateur local, le dossier starter et les licences. Le starter seul est également disponible dans `dqp-starter-v0.1.1.zip` ; il nécessite une installation préalable de dqp.

## Pour donner une référence à l’IA

Le document autonome est **dqp/docs/v1/AI_GUIDE.md**. Après publication d’un dépôt public et du tag, cette page aura une URL partageable. Le script `scripts/ai_link.py` produit l’URL exacte à partir de votre URL de dépôt. Aucun compte GitHub ou nom de dépôt n’a été supposé.

Version du logiciel : 0.1.1, expérimentale. Version du format de questionnaire : DQP 1.0. Les rapports 0.1.0 doivent être rejoués avec le moteur 0.1.0.
