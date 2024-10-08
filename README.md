# Générateur de mots de passe en Rust - Projet éducatif - Étape 1

## Introduction

Bienvenue dans ce projet éducatif de générateur de mots de passe en Rust ! Ce projet a pour but de démontrer les étapes de création d'un code propre, bien structuré et testé, en utilisant Rust comme langage de programmation.

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

## Étape 1 : Implémentation de base

Vous êtes actuellement sur la branche `etape1-code-fonctionnel`, qui représente la première étape de développement de notre projet : l'implémentation de base.

### Objectifs de cette étape

1. Créer la structure de base du projet Rust
2. Implémenter les fonctionnalités fondamentales de la bibliothèque de génération de mots de passe
3. Développer une interface en ligne de commande simple pour utiliser la bibliothèque
4. Assurer que le code est fonctionnel et produit les résultats attendus

### Points clés

- Le code doit être simple et direct, sans complications inutiles.
- Les fonctions doivent avoir une responsabilité unique et être faciles à comprendre.
- L'interface de la bibliothèque doit être intuitive et facile à utiliser.
- L'exécutable doit démontrer l'utilisation basique de la bibliothèque.

### Contenu clé

- `src/lib.rs` : Implémentation de la bibliothèque de génération de mots de passe
- `src/main.rs` : Code de l'interface en ligne de commande
- `Cargo.toml` : Configuration du projet et dépendances

### Fonctionnalités implémentées

- Génération de mots de passe aléatoires
- Personnalisation de la longueur du mot de passe
- Options pour inclure/exclure des types de caractères spécifiques
- Interface CLI basique pour utiliser la bibliothèque

### Comment utiliser

1. Clonez le dépôt et assurez-vous d'être sur la branche `etape1-code-fonctionnel`
2. Compilez le projet :
   ```
   cargo build
   ```
3. Exécutez le programme :
   ```
   cargo run -- --length 12
   ```

### Prochaines étapes

Pour voir l'ajout de la gestion des logs, passez à la branche `etape2-gestion-logs` :

```
git checkout etape2-gestion-logs
```

## Comment utiliser ce projet

1. Explorez chaque branche dans l'ordre pour voir l'évolution du projet.
2. Lisez les README spécifiques à chaque étape pour comprendre les changements et les nouveaux concepts introduits.
3. Examinez le code et les tests pour voir comment les meilleures pratiques sont appliquées.

## Contribution

Ce projet est principalement éducatif, mais les suggestions d'amélioration sont les bienvenues. N'hésitez pas à ouvrir une issue pour discuter de potentielles améliorations ou corrections.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
