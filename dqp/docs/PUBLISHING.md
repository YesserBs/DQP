# Publier DQP sur GitHub

Ce dépôt est préparé pour une version expérimentale **0.1.2**. La documentation est lisible directement sur GitHub ; aucun hébergement web séparé n’est nécessaire. Les actions décrites ci-dessous doivent être réalisées sur votre propre dépôt.

## 1. Préparer le dépôt

Si vous avez déjà poussé la version 0.1.0, copiez les fichiers de cette mise à jour dans votre dépôt en conservant votre dossier `.git` et vos modifications personnelles. Examinez le diff avant de committer. Ne remplacez pas le dépôt par le dossier `starter` : ce dernier est uniquement destiné aux utilisateurs.

Pour un nouveau dépôt, créez d’abord un dépôt GitHub vide, idéalement public pour que sa documentation soit accessible sans connexion aux outils IA. Initialisez le dossier contenant `Cargo.toml` :

```bash
git init -b main
git add .
git commit -m "Prepare DQP 0.1.2 public distribution"
```

Ajoutez ensuite l’URL réelle du dépôt comme remote `origin` avec `git remote add origin URL_DU_DEPOT`. Cette commande est inutile si `origin` existe déjà. Remplacez le libellé par l’URL réelle ; aucune URL de dépôt n’a été inventée dans cette livraison.

Pour un dépôt existant :

```bash
git status
git diff
cargo test --locked
cargo fmt --all -- --check
git add .
git commit -m "Prepare DQP 0.1.2 public distribution"
git push -u origin HEAD
```

Pour un nouveau dépôt, effectuez aussi les vérifications Cargo puis le push. Ne poussez ni `target/`, ni les fichiers de session personnels, ni les binaires locaux : le `.gitignore` fourni protège les chemins de sortie habituels. Les exemples de session fournis sont fictifs.

## 2. Vérifier la CI

Dans **Actions**, attendre le succès de `DQP checks`. Le workflow teste Linux, macOS et Windows. La vérification locale réalisée pour cette livraison couvre seulement Linux ; elle ne remplace pas cette CI.

## 3. Préparer une release

Le tag doit être identique à la version Cargo, précédée de `v` :

```bash
git tag -a v0.1.2 -m "DQP 0.1.2 experimental"
git push origin v0.1.2
```

Les workflows GitHub sont à la racine du dépôt (`.github/workflows/`), tandis que le projet Rust reste dans `dqp/`. Le `working-directory: dqp` permet aux commandes de trouver `Cargo.toml` et les scripts ; les chemins d’artefacts sont explicitement préfixés par `dqp/`.

Le workflow `Release packages` vérifie la version, lance les tests, compile et prépare quatre paquets natifs plus le starter. Il crée ensuite une **release en brouillon et marquée pre-release**, avec SHA256SUMS.txt et les notes.

Les quatre environnements sont Ubuntu 22.04 x86-64, Windows x86-64, macOS 15 ARM64 et macOS 15 Intel. Aucun serveur auto-hébergé ou secret personnalisé n’est requis ; le job de création utilise le jeton GitHub du workflow avec `contents: write`. Les politiques d’organisation peuvent imposer des restrictions supplémentaires.

Si un job échoue, aucun brouillon n’est créé par le job final. Corrigez le problème avant la publication. Pour une nouvelle version, mettez à jour Cargo.toml, Cargo.lock, les notes et les références documentaires, puis créez un nouveau tag. Ne déplacez pas silencieusement un tag public déjà utilisé.

Si vous relancez le workflow après la création réussie du brouillon, `gh release create` échouera car la release existe déjà. Consultez le brouillon existant ; pour remplacer ses assets, utilisez explicitement l’interface GitHub ou supprimez ce brouillon non publié avant de relancer.

## 4. Publier

Ouvrez le brouillon dans **Releases**, vérifiez les cinq archives, les sommes de contrôle et le statut des tests. Testez au moins le paquet de votre machine avec le starter. Publiez ensuite la pre-release depuis GitHub. Ce choix de brouillon sert à vérifier une distribution concrète avant sa publication.

## 5. Donner une seule URL à une IA

Une fois le tag poussé, générez le lien exact depuis votre URL de dépôt :

```bash
python3 dqp/scripts/ai_link.py URL_HTTPS_REELLE_DU_DEPOT
```

Sous Windows, utilisez `python` si `python3` n’est pas disponible. Le script affiche une URL épinglée au tag `v0.1.1` et un prompt prêt à adapter. Cette URL sera accessible si le dépôt est public et le tag existe. Vous pouvez également ouvrir `docs/v1/AI_GUIDE.md` sur GitHub, sélectionner le tag et copier le lien.

Une IA sans navigation ou sans accès au dépôt devra recevoir le contenu du guide. Publier le dépôt ou ajouter `llms.txt` ne garantit ni l’indexation ni la consultation automatique.

## 6. Construire un paquet local

Avec Rust, Cargo et Python disponibles :

```bash
cd dqp
cargo build --release --locked
python3 scripts/package.py --target x86_64-unknown-linux-gnu --binary target/release/dqp --out dist
```

La cible doit correspondre au binaire réellement compilé. Le script ne fait pas de compilation croisée. Pour le starter seul :

```bash
python3 scripts/package.py --starter-only --out dist
```

Les licences du runtime Rust utilisées pour la livraison locale sont conservées dans `packaging/licenses`. Lors d’une mise à jour majeure de l’outillage Rust, revérifiez ce dossier et les notices des dépendances.
