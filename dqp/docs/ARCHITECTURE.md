# Architecture et décisions

## Objectif

Séparer la création du scénario, son exécution reproductible et son analyse pédagogique. Une IA peut intervenir avant et après ; elle n’intervient pas dans la sélection des questions pendant la session.

## Modules

Le lecteur JSON produit une valeur et une table JSON Pointer → span source. Il s’appuie sur `serde_json` pour décoder correctement les scalaires (chaînes, échappements et nombres), tout en contrôlant les objets pour détecter les doublons avant qu’ils ne soient écrasés.

Le modèle typé refuse les champs inconnus. Le compilateur valide ensuite les références, les types des comparaisons et le graphe. Il résout les destinations textuelles en indices et normalise les valeurs par défaut. La détection de cycles utilise l’algorithme de Kahn ; l’accessibilité utilise un parcours itératif. Ces analyses sont linéaires dans la taille du graphe après construction des tables d’identifiants.

Le moteur possède un artefact validé, un pointeur courant, des compteurs et une trace. Son état est privé. Une soumission invalide ne modifie pas cet état. Aucun appel d’horloge, réseau, fichier ou terminal n’a lieu dans le moteur. Sa seule source de variation est la séquence d’entrées acceptées.

La CLI gère les fichiers, le clavier et les interruptions. Un fil de lecture et un canal borné permettent d’attendre une ligne tout en observant le signal Ctrl+C. La justification est demandée avant de valider l’étape entière. Les fichiers temporaires sont créés dans le même dossier que la destination. Les nouvelles destinations ne sont jamais écrasées ; les checkpoints suivants remplacent uniquement le rapport créé pour cette session.

Le rapport inclut un instantané de chaque question rencontrée. Les identifiants de règles et les compteurs rendent le chemin explicable. Le rejeu reconstruit ce rapport avec les mêmes entrées et le compare intégralement, hors métadonnées de transport et de temps.

## Choix structurants

| Décision | Motif | Coût assumé |
|---|---|---|
| Rust | types, erreurs explicites, exécutable local | apprentissage et compilation initiale |
| JSON comme syntaxe | édition et génération faciles | verbosité ; pas de commentaires |
| `.dqp.json` recommandé | identité et outils JSON existants | extension plus longue |
| Graphe acyclique | terminaison bornée et analyse simple | pas de retours sur la même question |
| Première règle vraie | priorité lisible et déterministe | l’ordre doit être relu attentivement |
| Compteurs entiers globaux | calcul sans ambiguïté | pas de note pondérée |
| Source dans l’artefact | diagnostic, identité et vérification | artefact plus volumineux |
| Recompilation au chargement | valider un artefact externe sans faire confiance à ses indices | pas de gain de temps au chargement |
| Réponses correctes dans le source | outil de révision transparent | pas un dispositif anti-triche |
| IA hors du moteur | indépendance et rejeu exact | analyse sémantique manuelle ou externe |

## Garanties et limites

Le déterminisme concerne le programme normalisé, le même moteur et les entrées acceptées. Les timestamps n’en font pas partie. Les tableaux gardent leur ordre ; les agrégations par notion sont des `BTreeMap` pour une sérialisation stable.

L’empreinte de source détecte un changement de fichier ; elle ne signe pas son auteur. Le rejeu prouve une cohérence interne, pas que les réponses ont été données par une personne donnée. Les corrections peuvent être erronées même lorsque la compilation réussit.

Les limites de taille et de profondeur bornent le travail normal du compilateur. Ce n’est pas un service exposé à des utilisateurs non fiables et il n’a pas subi d’audit de sécurité. Une ligne CLI peut être lue avant le rejet de sa longueur ; l’usage prévu est un terminal local, pas une entrée réseau non bornée.

La dernière réponse validée est sauvegardée après son calcul. Si l’écriture échoue, la CLI signale l’échec et conserve le dernier checkpoint réussi quand le système de fichiers le permet. `sync_all` sur le fichier temporaire ne garantit pas la durabilité du renommage en cas de panne matérielle : le dossier n’est pas explicitement synchronisé. La réponse en cours n’est pas récupérable automatiquement. Le rejeu d’un rapport partiel est supporté, la reprise interactive ne l’est pas encore.

## Évolutions possibles, par ordre d’intérêt

1. Tests génératifs et fuzzing du lecteur JSON, puis réduction des cas d’erreurs.
2. Complétion VS Code et validation depuis le JSON Schema, puis protocole LSP si utile.
3. Reprise interactive d’une session, avec vérification de version et rejeu préalable.
4. Compteurs par notion et groupes de questions, avec sémantique documentée.
5. Analyse de conditions impossibles ou de règles masquées, clairement présentée comme diagnostic supplémentaire.
6. API web utilisant le même moteur, puis compilation WebAssembly à évaluer.
7. Exports d’interopérabilité QTI/xAPI limités à des sous-ensembles documentés.
8. Profil narratif distinct (choix sans correction, variables typées) après stabilisation du profil pédagogique.

Ces éléments ne sont pas implémentés. Introduire des boucles ou du code arbitraire changerait fortement les garanties : ces fonctionnalités ne doivent pas être ajoutées par simple commodité.
