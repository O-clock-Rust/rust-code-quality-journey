# Générateur de mots de passe en Rust - Projet éducatif - Étape 2

## Introduction

Bienvenue dans ce projet éducatif de générateur de mots de passe en Rust ! Ce projet démontre les étapes de création d'un code propre, bien structuré et testé, en utilisant Rust comme langage de programmation.

Le projet consiste en une bibliothèque de génération de mots de passe et une interface en ligne de commande (CLI) pour l'utiliser. À travers ce projet, nous explorerons les meilleures pratiques de développement, de la conception initiale aux tests approfondis.

## Structure du projet

Ce projet est organisé en plusieurs branches Git, chacune représentant une étape spécifique du processus de développement :

- `main` (Étape 0) : Conception du projet
- `etape1-code-fonctionnel` : Implémentation de base
- `etape2-gestion-logs` : Ajout de la gestion des logs
- `etape3-gestion-erreurs` : Implémentation de la gestion des erreurs avec `anyhow`
- `etape4-documentation` : Ajout de la documentation complète
- `etape5-tests` : Implémentation des tests unitaires et d'intégration

Chaque branche construit sur la précédente, ajoutant de nouvelles fonctionnalités ou améliorations.

## Étape 2 : Gestion des logs

Vous êtes actuellement sur la branche `etape2-gestion-logs`, qui représente la deuxième étape de développement de notre projet : l'ajout de la gestion des logs.

### Objectifs de cette étape

1. Intégrer un système de logging dans la bibliothèque et l'application CLI
2. Ajouter des logs pertinents pour suivre le flux d'exécution et faciliter le débogage
3. Configurer différents niveaux de log (info, debug) pour différents contextes d'utilisation
4. Améliorer la maintenabilité et la débuggabilité du code

### Points clés

- Les logs doivent être simples, concis et pertinents
- Utiliser les logs de type 'info' pour les événements importants du flux principal
- Utiliser les logs de type 'debug' pour les détails utiles lors du débogage
- Éviter le logging excessif qui pourrait nuire aux performances
- Configurer les logs pour qu'ils soient facilement activables/désactivables

### Contenu clé

- `src/lib.rs` : Ajout de logs dans la bibliothèque de génération de mots de passe
- `src/main.rs` : Intégration des logs dans l'interface en ligne de commande
- `Cargo.toml` : Ajout des dépendances pour la gestion des logs (par exemple, `log` et `env_logger`)

### Fonctionnalités ajoutées

- Logging des étapes clés de la génération de mots de passe
- Logging des actions de l'utilisateur dans l'interface CLI
- Configuration des niveaux de log via des variables d'environnement

### Comment utiliser

1. Clonez le dépôt et assurez-vous d'être sur la branche `etape2-gestion-logs`
2. Compilez le projet :
   ```
   cargo build
   ```
3. Exécutez le programme avec logging activé :
   ```
   cargo run -- --length 12
   ```

### Prochaines étapes

Pour voir l'ajout de la gestion des erreurs, passez à la branche `etape3-gestion-erreurs` :

```
git checkout etape3-gestion-erreurs
```

## Contribution

Ce projet est principalement éducatif, mais les suggestions d'amélioration sont les bienvenues. N'hésitez pas à ouvrir une issue pour discuter de potentielles améliorations ou corrections.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
