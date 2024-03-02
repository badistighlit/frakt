# Projet Fractal 

## Introduction
Ce projet vise à créer un rendu fractal distribué en utilisant Rust et la communication entre un serveur et des travailleurs.

## Structure du Code
La structure du code est organisée en plusieurs parties, notamment la définition des types de messages, le calcul des fragments, la communication réseau, etc.

### Crates de Message
La crate de messages contient les types de messages nécessaires à la communication entre le serveur et les travailleurs. Voici un aperçu des principales structures :
- `JuliaDescriptor`: Structure décrivant les propriétés d'un ensemble de Julia.
- `FragmentTask`: Structure représentant une tâche de calcul de fragment fractal.
- `FragmentRequest`: Structure envoyée par un travailleur pour demander une tâche au serveur.
- `FragmentResult`: Structure contenant les résultats d'une tâche de calcul de fragment.
- `Message`: Enumération de différents types de messages.

### Crates de Worker
La crate du worker contient la logique du côté du travailleur, notamment l'envoi et la réception de messages, le calcul des fragments, etc. Voici un aperçu des principales fonctions et structures :
- `send_message`: Fonction pour envoyer un message au serveur.
- `reception_message`: Fonction pour recevoir et traiter un message du serveur.
- `calculate_fragment`: Fonction pour calculer un fragment fractal à partir d'une tâche donnée.
- `main`: Fonction principale du travailleur, établissant une connexion avec le serveur et envoyant une demande de travail.

## Utilisation



# Pour lancer un travailleur, utilisez la commande suivante :


cargo run --bin worker

# Communication
La communication entre le serveur et les travailleurs se fait via TCP. Les messages sont sérialisés en JSON pour la facilité de transmission.

# Bonus


# Notes de l'Équipe
Chaque contributeur est identifié dans les commits du dépôt.
La documentation Markdown met en évidence notre démarche d'élaboration des composants, les spécificités du projet, et les bonus implémentés.
