# fluxum

Exo Rust
Job runner ?¿

Le but est de créer une CLI, qui va permettre de lancer des tâches arbitraires, un peu comme un système de CI/CD.

Il doit se baser sur un fichier yaml, décrivant des tâches à effectuer, leur paramètres, et leur ordre d'execution.

Il doit respecter au maximum le principe de « convention over configuration »

L'objectif est de pouvoir le lancer dans un dossier sans paramètres, avec un fichier de configuration conventionné.

Features basiques:
- Execution de tâches arbitraires Shell
- Executer les tâches dans un contexte local
- Logger les sorties
- Gérer le lifecycle du « pipeline » en fonction des sorties

Features bonus
- Autre types de tâches, « Python script runner » / « Go builder » / « Docker builder »
- Gérer l'execution en parallèle de tâches (fork et join des branches du pipeline)
- Gérer les tâches « optionnelles », ou « fail-able »

Requirements:
- Doit passer sans warnings cargo check / build / fmt / clippy
- Doit utiliser le crate tokio, et le main doit être un process tokio