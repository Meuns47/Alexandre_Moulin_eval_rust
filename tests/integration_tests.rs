use std::time::{Duration, Instant};
use lru_cache::cache::Cache;
use lru_cache::lru_cache_trait::LRUCache;
use lru_cache::mock_api::MockApi;

#[derive(Debug)]
struct CacheOperation {
    key: String,
    source: String,
    duration_ns: u64,
    cache_state: String,
}

fn format_duration(duration_ns: u64) -> String {
    if duration_ns >= 1_000_000 {
        format!("{} ms", duration_ns / 1_000_000)
    } else {
        format!("{} µs", duration_ns / 1_000)
    }
}

fn print_scenario(operations: &[CacheOperation]) {
    println!("\n{:-<80}", "");
    println!("{:<15} {:<15} {:<15} {:<30}", "Clé", "Source", "Durée", "État Cache");
    println!("{:-<80}", "");
    
    for op in operations {
        println!("{:<15} {:<15} {:<15} {:<30}",
            op.key,
            op.source,
            format_duration(op.duration_ns),
            op.cache_state
        );
    }
    println!("{:-<80}", "");
}

#[test]
fn test_basic_cache_scenario() {
    let api = MockApi::new(Duration::from_millis(100));
    let mut cache = Cache::new(3);
    let mut operations = Vec::new();
    
    // Test du scénario de base
    let keys = ["A", "B", "C", "D", "B", "A"];
    
    for key in keys {
        let start = Instant::now();
        let key_string = key.to_string();
        
        if let Some(_value) = cache.get(&key_string) {
            operations.push(CacheOperation {
                key: key_string,
                source: "CACHE".to_string(),
                duration_ns: start.elapsed().as_nanos() as u64,
                cache_state: format!("Cache: [{}]", cache.get_state()),
            });
        } else {
            let value = api.get_data(key);
            let duration = start.elapsed().as_nanos() as u64;
            cache.put(key_string.clone(), value);
            
            operations.push(CacheOperation {
                key: key_string,
                source: "API".to_string(),
                duration_ns: duration,
                cache_state: format!("Cache: [{}]", cache.get_state()),
            });
        }
    }
    
    print_scenario(&operations);
}

#[test]
fn test_persistent_cache() {
    let cache_file = "integration_cache.txt";
    let api = MockApi::new(Duration::from_millis(100));
    let mut operations = Vec::new();
    
    // S'assurer que le cache est vide au début
    if std::path::Path::new(cache_file).exists() {
        std::fs::remove_file(cache_file).unwrap();
    }
    
    // Première phase : remplir le cache
    {
        let mut cache = Cache::new_persistent(3, cache_file);
        let keys = ["A", "B", "C", "D"];
        
        for key in keys {
            let start = Instant::now();
            let key_string = key.to_string();
            let value = api.get_data(key);
            let duration = start.elapsed().as_nanos() as u64;
            cache.put(key_string.clone(), value);
            
            operations.push(CacheOperation {
                key: key_string,
                source: "API".to_string(),
                duration_ns: duration,
                cache_state: format!("Cache: [{}]", cache.get_state()),
            });
        }
    }
    
    // Deuxième phase : utiliser le cache persistant
    {
        let mut cache = Cache::new_persistent(3, cache_file);
        let keys = ["D", "C", "B", "A"];
        
        for key in keys {
            let start = Instant::now();
            let key_string = key.to_string();
            
            if let Some(_value) = cache.get(&key_string) {
                operations.push(CacheOperation {
                    key: key_string,
                    source: "CACHE".to_string(),
                    duration_ns: start.elapsed().as_nanos() as u64,
                    cache_state: format!("Cache: [{}]", cache.get_state()),
                });
            } else {
                let value = api.get_data(key);
                let duration = start.elapsed().as_nanos() as u64;
                cache.put(key_string.clone(), value);
                
                operations.push(CacheOperation {
                    key: key_string,
                    source: "API".to_string(),
                    duration_ns: duration,
                    cache_state: format!("Cache: [{}]", cache.get_state()),
                });
            }
        }
    }
    
    print_scenario(&operations);
    println!("\nLe cache est sauvegardé dans : {}", cache_file);
}
