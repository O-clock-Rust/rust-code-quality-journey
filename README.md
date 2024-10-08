# Générateur de mots de passe en Rust - Projet éducatif - Étape 5

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

## Étape 5 : Tests unitaires et d'intégration

Vous êtes actuellement sur la branche `etape5-tests`, qui représente la cinquième et dernière étape de développement de notre projet : l'implémentation des tests unitaires et d'intégration.

### Objectifs de cette étape

1. Implémenter des tests unitaires pour les fonctions individuelles de la bibliothèque
2. Créer des tests d'intégration pour vérifier le fonctionnement de l'application dans son ensemble
3. Assurer une couverture de test adéquate pour le projet
4. Démontrer l'utilisation de différentes techniques de test en Rust

### Points clés

- Rester simple et efficace dans l'écriture des tests
- Distinguer clairement les tests unitaires des tests d'intégration :
  - Tests unitaires : se concentrent sur des fonctions ou méthodes individuelles
  - Tests d'intégration : vérifient le fonctionnement de plusieurs composants ensemble
- Écrire des tests lisibles et maintenables
- Utiliser des noms de tests descriptifs qui expliquent ce qui est testé
- Éviter la duplication de code dans les tests en utilisant des helpers ou des macros si nécessaire
- Tester les cas normaux et les cas limites
- Utiliser les fonctionnalités de test de Rust comme `#[test]`, `#[should_panic]`, etc.

### Contenu clé

- `src/lib.rs` : Tests unitaires pour la bibliothèque
- `tests/` : Dossier contenant les tests d'intégration
- `tests/cli.rs` : Tests d'intégration pour l'interface en ligne de commande

### Fonctionnalités ajoutées

- Tests unitaires pour chaque fonction principale de la bibliothèque
- Tests d'intégration pour vérifier le comportement de l'application CLI
- Utilisation de `assert_cmd` et `predicates` pour les tests d'intégration CLI

### Comment utiliser

1. Clonez le dépôt et assurez-vous d'être sur la branche `etape5-tests`
2. Exécutez tous les tests :
   ```
   cargo test
   ```
3. Pour exécuter uniquement les tests unitaires :
   ```
   cargo test --lib
   ```
4. Pour exécuter uniquement les tests d'intégration :
   ```
   cargo test --test '*'
   ```

### Exemple de test unitaire

```rust
#[test]
fn test_password_length() {
    let generator = PasswordGenerator::new(10).unwrap();
    let password = generator.generate().unwrap();
    assert_eq!(password.len(), 10);
}
```

### Exemple de test d'intégration

```rust
#[test]
fn test_cli_generate_password() {
    Command::cargo_bin("password-generator-cli")
        .unwrap()
        .arg("--length")
        .arg("12")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated password:"));
}
```

## Contribution

Ce projet est principalement éducatif, mais les suggestions d'amélioration sont les bienvenues. N'hésitez pas à ouvrir une issue pour discuter de potentielles améliorations ou corrections.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
