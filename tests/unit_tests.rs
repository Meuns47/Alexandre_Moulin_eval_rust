use lru_cache::cache::Cache;
use lru_cache::lru_cache_trait::LRUCache;
use std::fs;

#[test]
fn test_cache_basic() {
    let cache_file = "test_cache_basic.txt";
    let mut cache = Cache::new_persistent(3, cache_file);
    
    cache.put("A".to_string(), "data_A".to_string());
    assert_eq!(cache.get(&"A".to_string()), Some("data_A".to_string()));
    
    fs::remove_file(cache_file).unwrap_or(());
}

#[test]
fn test_cache_capacity() {
    let cache_file = "test_cache_capacity.txt";
    let mut cache = Cache::new_persistent(3, cache_file);
    
    cache.put("A".to_string(), "data_A".to_string());
    cache.put("B".to_string(), "data_B".to_string());
    cache.put("C".to_string(), "data_C".to_string());
    cache.put("D".to_string(), "data_D".to_string());
    
    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"B".to_string()), Some("data_B".to_string()));
    assert_eq!(cache.get(&"C".to_string()), Some("data_C".to_string()));
    assert_eq!(cache.get(&"D".to_string()), Some("data_D".to_string()));
    
    fs::remove_file(cache_file).unwrap_or(());
}

#[test]
fn test_cache_update_order() {
    let cache_file = "test_cache_order.txt";
    let mut cache = Cache::new_persistent(3, cache_file);
    
    cache.put("A".to_string(), "data_A".to_string());
    cache.put("B".to_string(), "data_B".to_string());
    cache.put("C".to_string(), "data_C".to_string());
    
    // Accès à A le rend plus récent
    cache.get(&"A".to_string());
    
    // D remplace B car A est plus récent et C aussi
    cache.put("D".to_string(), "data_D".to_string());
    
    assert_eq!(cache.get(&"A".to_string()), Some("data_A".to_string()));
    assert_eq!(cache.get(&"B".to_string()), None);
    assert_eq!(cache.get(&"C".to_string()), Some("data_C".to_string()));
    assert_eq!(cache.get(&"D".to_string()), Some("data_D".to_string()));
    
    fs::remove_file(cache_file).unwrap_or(());
}

#[test]
fn test_cache_with_pure_numbers() {
    // Cache en mémoire avec des nombres directement
    let mut cache: Cache<i32, f64> = Cache::new(3);
    
    cache.put(1, 1.5);
    cache.put(2, 2.5);
    cache.put(3, 3.5);
    cache.put(4, 4.5);
    
    assert_eq!(cache.get(&1), None);  // 1 devrait être évincé
    assert_eq!(cache.get(&2), Some(2.5));
    assert_eq!(cache.get(&3), Some(3.5));
    assert_eq!(cache.get(&4), Some(4.5));
}
