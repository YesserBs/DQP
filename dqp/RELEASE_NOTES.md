DQP permet de lancer des questionnaires à embranchements depuis un JSON et d’obtenir un rapport autonome avec réponses, justifications et raisons du parcours.

## 0.1.2

Cette version ajoute la reprise des sessions interrompues avec `dqp continue rapport.json`. Le rapport mémorise le chemin exact du scénario source et la reprise vérifie que le scénario et toutes les réponses déjà enregistrées correspondent avant de poursuivre.

Cette version expérimentale ajoute le parcours utilisateur, une documentation autonome pour les IA et des paquets d’installation. Le moteur pédagogique reste inchangé.

Téléchargez le paquet de votre plateforme ; le dossier starter est inclus. Rust/Cargo ne sont pas nécessaires pour utiliser le binaire précompilé. Le starter seul exige une installation préalable de la commande dqp.

Limites : QCM à réponse unique, parcours sans boucle, justification monoligne ; aucune IA intégrée, signature anti-triche ou certification pédagogique. Les paquets macOS ne sont pas notarisés. Les rapports doivent être rejoués avec la même version du moteur.

Consultez INSTALLATION.md et les fichiers SHA256SUMS.txt. Les tests de la release doivent réussir sur chaque plateforme avant publication.
