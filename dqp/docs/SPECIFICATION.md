# Spécification DQP 1.0

Ce document définit le comportement implémenté dans DQP 0.1.2. Les structures Rust et les tests constituent la référence exécutable. Le JSON Schema facilite l’édition mais ne remplace pas les vérifications du compilateur.

## 1. Source

Encodage UTF-8, JSON strict, sans commentaires, virgules terminales ou BOM. Clés dupliquées rejetées, y compris si leur écriture échappée diffère (`a` et `\u0061`). Les clés inconnues sont des erreurs. L’ordre des propriétés d’un objet ne définit aucune priorité ; l’ordre des tableaux est significatif.

Source : 4 Mio maximum. Questions : 1 à 256. Choix par question : 2 à 26. Les fichiers source, artefact et rapport doivent respecter la limite globale de lecture de 32 Mio et une profondeur JSON de 64. Une chaîne vide ou composée uniquement d’espaces est refusée pour le titre, la description, l’énoncé, les propositions et l’explication.

Identifiants : `[A-Za-z0-9_-]{1,80}`, sensibles à la casse. Les identifiants de questions sont uniques dans le scénario ; les identifiants de choix et de règles sont uniques dans leur question. `end` est réservé comme destination terminale et ne peut être un identifiant de question. L’identifiant du scénario est une chaîne descriptive, pas une identité mondiale.

### Objet scénario

| Champ | Type | Obligation |
|---|---|---|
| `dqp_version` | chaîne `1.0` | obligatoire |
| `id` | identifiant | obligatoire |
| `title` | texte | obligatoire |
| `description` | texte | obligatoire |
| `start` | identifiant de question existant | obligatoire |
| `settings` | objet | défaut `{}` |
| `questions` | tableau de questions | obligatoire |

`settings.feedback` vaut `end` (défaut) ou `immediate`. Aucun autre réglage n’est accepté.

### Question

`id`, `prompt`, `choices`, `answer`, `next` sont obligatoires. `concepts` est un tableau facultatif de textes non vides et uniques ; défaut `[]`. `routes` est un tableau facultatif ; défaut `[]`.

Un choix contient `id` et `text`. `answer` contient `choice` (un choix existant de la même question) et `explanation` (texte non vide). `next` désigne une question existante ou `end`.

Une route contient `id`, `when`, `goto`. `goto` suit les mêmes règles que `next`.

## 2. Conditions

Formes possibles :

```json
{"kind":"compare","metric":"correct_count","operator":"eq","value":0}
{"kind":"all","conditions":[{"kind":"compare","metric":"answered_count","operator":"gte","value":3}]}
{"kind":"any","conditions":[{"kind":"compare","metric":"last_answer.correct","operator":"eq","value":false}]}
{"kind":"not","condition":{"kind":"compare","metric":"last_answer.correct","operator":"eq","value":true}}
```

`all` et `any` exigent au moins un enfant ; `not` exactement un. Pas de code embarqué, variables utilisateur, fonctions ou références à une réponse antérieure arbitraire.

| Mesure | Type de valeur | Opérateurs |
|---|---|---|
| `last_answer.correct` | booléen | `eq`, `ne` |
| `last_answer.choice` | identifiant d’un choix de la question courante | `eq`, `ne` |
| `answered_count` | entier non négatif, au plus i64::MAX | `eq`, `ne`, `lt`, `lte`, `gt`, `gte` |
| `correct_count` | même type | mêmes opérateurs |
| `consecutive_errors` | même type | mêmes opérateurs |

`3.0` n’est pas un entier accepté par le compilateur, même si sa valeur mathématique est entière. Le JSON Schema peut accepter cette représentation : la validation DQP fait autorité. Aucune conversion implicite de chaîne, booléen ou nombre n’a lieu.

Les compteurs portent sur toute la session. `consecutive_errors` devient zéro après une bonne réponse. La justification ne participe à aucune condition.

## 3. Ordre normatif d’une étape

1. Vérifier que la session n’est pas terminée.
2. Valider le choix et la longueur de la justification sans modifier l’état.
3. Calculer la correction du choix selon `answer.choice`.
4. Incrémenter `answered_count`, éventuellement `correct_count`, puis mettre à jour `consecutive_errors`.
5. Évaluer les routes dans leur ordre, avec les nouveaux compteurs. S’arrêter à la première vraie.
6. Utiliser `next` si aucune route ne correspond.
7. Ajouter une étape à la trace et déplacer le pointeur courant.
8. La CLI sauvegarde le rapport, puis affiche la correction si le mode est `immediate`.

Une bonne réponse vaut un point. Les identifiants sont comparés exactement, sans normalisation Unicode. La CLI retire les espaces autour de l’identifiant saisi ; l’API ne le fait pas. La justification est conservée telle quelle, hors fin de ligne supprimée par la CLI. Entrée seule produit `null` ; une chaîne d’espaces reste une justification présente. `:quit` est réservé par la CLI, pas par l’API.

Une réponse n’est acceptée qu’après la saisie de la justification ou son omission explicite par Entrée. EOF ou interruption avant cette étape ne compte pas le choix provisoire.

## 4. Graphe

Les transitions conditionnelles et de repli forment un graphe dirigé. Le compilateur exige :

- chaque destination existe ou vaut `end` ;
- aucune composante du graphe ne comporte de cycle ;
- chaque question est structurellement accessible depuis `start`.

L’analyse considère toutes les routes, même celles dont les conditions pourraient être impossibles. Elle ne prouve ni l’exclusivité ni la satisfaisabilité des conditions. Les chevauchements sont autorisés et résolus par priorité ordonnée. Aucune détection générale de règles masquées n’est implémentée dans cette version.

Un parcours validé visite au plus 256 questions. La terminaison du parcours suppose que l’utilisateur fournisse les entrées ; l’attente de saisie n’a pas de délai maximal.

## 5. Compilation et artefact

Le fichier `.dqpc` est un objet JSON contenant `artifact_version`, `source_sha256`, `source`, `program`.

Le programme comporte le scénario typé et normalisé (valeurs par défaut explicites), l’indice d’entrée, et pour chaque question les indices de destination conditionnels et de repli (`null` signifie la fin). Les conditions sont représentées par des variantes typées. Il ne s’agit pas de code machine.

L’empreinte SHA-256 porte sur les octets UTF-8 exacts du source, espaces compris. Une mise en forme du source change donc son identité pour le rejeu. Le source embarqué conserve les positions originales des diagnostics ; la table de spans est reconstruite au chargement si nécessaire.

Le chargement recompile le source embarqué et compare tout l’artefact à celui reconstruit. Il refuse les versions inconnues et les incohérences. Un attaquant peut modifier simultanément le source et recompiler : ce contrôle garantit la cohérence, pas la provenance ou l’authenticité.

## 6. Rapport

Le fichier CLI contient :

- `result` : résultat déterministe de type `Report` ;
- `metadata` : dates Unix en millisecondes, motif de fin, identifiants des corrections écrites au terminal.

`result` inclut les versions, l’empreinte du source, l’identité et l’objectif du scénario, le mode de correction, `status`, `next_question`, les compteurs, le bilan par notion et les étapes.

`status` vaut `completed` si le pointeur est terminal ; sinon `interrupted`. Pendant une session en cours, le checkpoint est donc un préfixe rejouable avec `status: interrupted` et `metadata.reason: in_progress`.

Chaque étape contient : son numéro (à partir de 1), la question complète, le choix et la justification, le résultat, les compteurs après correction, les règles effectivement examinées avec leur résultat booléen, la règle choisie ou `null`, la destination, et les corrections prévues auparavant.

`feedback_after_answer` décrit la politique du moteur. `prior_feedback` liste les corrections immédiates planifiées après les étapes précédentes. `metadata.feedback_written` distingue les corrections dont l’écriture terminal a réussi. En cas d’échec de sortie ou de panne, politique prévue et affichage réel peuvent diverger. Aucune donnée ne prouve la lecture par l’utilisateur.

Le bilan par notion compte une question pour chacune de ses notions. Ses totaux ne doivent pas être additionnés pour retrouver le total de session. Aucune conclusion pédagogique ou estimation de maîtrise n’est calculée par le moteur.

## 7. Rejeu

Le rejeu repart de l’état initial, réinjecte chaque `input` puis compare la totalité du rapport déterministe reconstruit. Il vérifie également les versions, les énoncés, corrections, états, compteurs et transitions enregistrés. Une session partielle ou vide est valable. Un changement de version logicielle peut rendre le rapport incompatible ; aucune migration automatique n’est proposée en V1.

Les métadonnées de temps et d’écriture terminal ne sont pas vérifiées par rejeu. Le rejeu ne constitue pas une signature ou une preuve anti-triche.

## 8. Diagnostics et codes de sortie

Les diagnostics contiennent `code`, `message`, `path` (JSON Pointer), `line`, `column`, `offset`, `length`. Lignes et colonnes commencent à 1 ; colonnes en caractères Unicode, offsets et longueurs en octets UTF-8. Pour un champ manquant, la position désigne le plus proche objet existant. Certains diagnostics structurels peuvent désigner l’objet de condition plutôt que son sous-champ.

| Codes | Sens |
|---|---|
| E001 | limite de taille |
| E100, E101, E102 | syntaxe JSON, clé dupliquée, profondeur |
| E110 | structure ou champ invalide |
| E200–E202 | version, identifiant ou contenu invalide |
| E203–E205 | doublon, choix inconnu, destination inconnue |
| E206, E207 | cycle, question inaccessible |
| E210–E212 | type, opérateur ou composition invalide |
| E220, E221 | artefact incompatible ou incohérent |
| E300–E302 | session terminée, choix invalide, justification trop longue |
| E310 | divergence de rejeu |
| E400 | erreur d’entrée/sortie |
| I001 | interruption avec sauvegarde partielle |

Tous les codes portent le préfixe `DQP-`. Codes de sortie CLI : 0 succès ; 2 validation (ou arguments CLI invalides) ; 3 rejeu/exécution incohérent ; 4 entrée/sortie ; 130 session interrompue. `--diagnostics json` concerne les diagnostics DQP sur stderr ; les erreurs d’arguments restent gérées par Clap.

Les erreurs de syntaxe arrêtent la lecture au premier défaut. La vérification sémantique agrège les erreurs indépendantes, puis l’analyse de graphe ne démarre que si les références sont valides. Aucun fichier compilé n’est produit en cas d’échec.
