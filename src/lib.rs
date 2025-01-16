//! # Cache LRU
//! 
//! Un cache qui garde en mémoire les N derniers éléments utilisés.
//! Quand le cache est plein, l'élément le moins récemment utilisé est supprimé.
//! 
//! Exemple avec un cache de taille 3 :
//! ```no_run
//! use lru_cache::cache::Cache;
//! use lru_cache::lru_cache_trait::LRUCache;
//! 
//! let mut cache = Cache::new(3);
//! cache.put("A".to_string(), "data_A".to_string());
//! cache.put("B".to_string(), "data_B".to_string());
//! cache.put("C".to_string(), "data_C".to_string());
//! 
//! // D évince A car le cache est plein
//! cache.put("D".to_string(), "data_D".to_string());
//! assert_eq!(cache.get(&"A".to_string()), None);
//! ```
//! 
//! Le cache peut aussi être persistant :
//! ```no_run
//! use lru_cache::cache::Cache;
//! use lru_cache::lru_cache_trait::LRUCache;
//! 
//! let mut cache = Cache::new_persistent(3, "cache.txt");
//! cache.put("key".to_string(), "value".to_string());
//! ```

pub mod lru_cache_trait;
pub mod cache;
pub mod mock_api;
