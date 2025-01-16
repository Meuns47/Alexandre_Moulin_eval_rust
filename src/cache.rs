use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};
use std::fmt::Debug;
use crate::lru_cache_trait::LRUCache;

/// Cache LRU avec option de persistance fichier.
/// 
/// # Exemple
/// ```no_run
/// use lru_cache::cache::Cache;
/// use lru_cache::lru_cache_trait::LRUCache;
/// 
/// // En mémoire
/// let mut cache = Cache::new(2);
/// cache.put("key1".to_string(), 42);
/// 
/// // Avec fichier
/// let mut cache = Cache::new_persistent(2, "cache.txt");
/// cache.put("key2".to_string(), 43);
/// ```
pub struct Cache<K, V> {
    capacity: usize,
    file_path: String,
    order: VecDeque<K>,
    data: HashMap<K, V>,
}

impl<K, V> Cache<K, V> 
where
    K: Clone + ToString + FromStr + Eq + std::hash::Hash,
    V: Clone + ToString + FromStr,
    <K as FromStr>::Err: Debug,
    <V as FromStr>::Err: Debug,
{
    /// Crée un cache qui persiste dans un fichier.
    pub fn new_persistent(capacity: usize, file_path: &str) -> Self {
        let mut cache = Cache {
            capacity,
            file_path: file_path.to_string(),
            order: VecDeque::with_capacity(capacity),
            data: HashMap::with_capacity(capacity),
        };
        cache.load_from_file();
        cache
    }

    /// Charge les données depuis le fichier.
    fn load_from_file(&mut self) {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(Result::ok) {
                if let Some((key, value)) = line.split_once('=') {
                    if let (Ok(key), Ok(value)) = (K::from_str(key), V::from_str(value)) {
                        self.put(key, value);
                    }
                }
            }
        }
    }

    /// Sauvegarde les données dans le fichier.
    fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.file_path)?;
        for key in &self.order {
            if let Some(value) = self.data.get(key) {
                writeln!(file, "{}={}", key.to_string(), value.to_string())?;
            }
        }
        Ok(())
    }

    /// Retourne l'état du cache pour debug.
    pub fn get_state(&self) -> String {
        self.order.iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<K, V> LRUCache<K, V> for Cache<K, V>
where
    K: Clone + ToString + FromStr + Eq + std::hash::Hash,
    V: Clone + ToString + FromStr,
    <K as FromStr>::Err: Debug,
    <V as FromStr>::Err: Debug,
{
    fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            file_path: String::new(),
            order: VecDeque::with_capacity(capacity),
            data: HashMap::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.data.get(key).cloned() {
            if let Some(pos) = self.order.iter().position(|k| k == key) {
                self.order.remove(pos);
                self.order.push_back(key.clone());
            }
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(pos) = self.order.iter().position(|k| k == &key) {
            self.order.remove(pos);
        } else if self.order.len() >= self.capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.data.remove(&old_key);
            }
        }
        
        self.order.push_back(key.clone());
        self.data.insert(key, value);
        
        if !self.file_path.is_empty() {
            self.save_to_file().ok();
        }
    }
}
