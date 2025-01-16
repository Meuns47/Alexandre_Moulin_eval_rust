# Cache LRU (Least Recently Used)

Une implémentation d'un cache LRU (Least Recently Used) en Rust avec persistance sur disque. Le cache maintient un nombre limité d'éléments et supprime automatiquement les éléments les moins récemment utilisés lorsqu'il atteint sa capacité maximale.

## Fonctionnalités

- Stockage clé-valeur avec taille maximale configurable
- Éviction automatique des éléments les moins récemment utilisés (LRU)
- Persistance des données sur disque
- Support pour différents types de données (génériques)
- Interface simple et intuitive
- Gestion efficace de la mémoire

## Utilisation

```rust
use lru_cache::cache::Cache;
use lru_cache::lru_cache_trait::LRUCache;

// Création d'un cache persistant de taille 3
let mut cache = Cache::new_persistent(3, "cache.txt");

// Ajout d'éléments
cache.put("A".to_string(), "data_A".to_string());
cache.put("B".to_string(), "data_B".to_string());
cache.put("C".to_string(), "data_C".to_string());

// L'ajout d'un nouvel élément évince l'élément le moins récemment utilisé
cache.put("D".to_string(), "data_D".to_string());
assert_eq!(cache.get(&"A".to_string()), None); // A a été évincé

// Les données persistent entre les redémarrages
let mut new_cache = Cache::new_persistent(3, "cache.txt");
assert_eq!(new_cache.get(&"D".to_string()), Some("data_D".to_string()));
```

## Structure

Le cache utilise une combinaison de `HashMap` pour un accès rapide aux données et `VecDeque` pour maintenir l'ordre LRU. La persistance est gérée via des opérations de fichier standard.

## Licence

Ce projet est sous licence MIT.
