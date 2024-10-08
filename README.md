# Générateur de mots de passe en Rust - Projet éducatif - Étape 3

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

## Étape 3 : Gestion des erreurs

Vous êtes actuellement sur la branche `etape3-gestion-erreurs`, qui représente la troisième étape de développement de notre projet : l'implémentation de la gestion des erreurs avec `anyhow`.

### Objectifs de cette étape

1. Intégrer la bibliothèque `anyhow` pour une gestion des erreurs plus robuste et flexible
2. Remplacer les types d'erreur personnalisés par des erreurs `anyhow`
3. Améliorer la propagation des erreurs à travers les différentes couches de l'application
4. Fournir des messages d'erreur plus informatifs et utiles pour les utilisateurs

### Points clés

- Utiliser `anyhow::Result` pour simplifier la gestion des erreurs
- Créer des erreurs personnalisées avec des contextes clairs et informatifs
- Propager les erreurs de manière cohérente à travers l'application
- Assurer que les messages d'erreur sont utiles pour le débogage et compréhensibles pour l'utilisateur final
- Maintenir un équilibre entre la gestion détaillée des erreurs et la simplicité du code

### Contenu clé

- `src/lib.rs` : Implémentation de la gestion des erreurs dans la bibliothèque
- `src/main.rs` : Gestion des erreurs dans l'interface en ligne de commande
- `Cargo.toml` : Ajout de la dépendance `anyhow`

### Fonctionnalités ajoutées

- Utilisation de `anyhow::Result` pour les fonctions retournant des résultats
- Messages d'erreur contextuels et plus détaillés
- Meilleure gestion des cas d'erreur dans l'interface CLI

### Comment utiliser

1. Clonez le dépôt et assurez-vous d'être sur la branche `etape3-gestion-erreurs`
2. Compilez le projet :
   ```
   cargo build
   ```
3. Exécutez le programme :
   ```
   cargo run -- --length 12
   ```
4. Pour voir la gestion des erreurs en action, essayez des cas d'erreur, par exemple :
   ```
   cargo run -- --length 0
   ```

### Prochaines étapes

Pour voir l'ajout de la documentation complète, passez à la branche `etape4-documentation` :

```
git checkout etape4-documentation
```

## Contribution

Ce projet est principalement éducatif, mais les suggestions d'amélioration sont les bienvenues. N'hésitez pas à ouvrir une issue pour discuter de potentielles améliorations ou corrections.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
