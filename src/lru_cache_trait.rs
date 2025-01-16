/// Trait pour implémenter un cache LRU.
/// 
/// # Exemple
/// ```no_run
/// use lru_cache::lru_cache_trait::LRUCache;
/// use lru_cache::cache::Cache;
/// 
/// let mut cache = Cache::new(2);
/// cache.put("A".to_string(), 1);
/// cache.put("B".to_string(), 2);
/// 
/// // A est encore dans le cache
/// assert_eq!(cache.get(&"A".to_string()), Some(1));
/// 
/// // C évince B car A a été lu plus récemment
/// cache.put("C".to_string(), 3);
/// assert_eq!(cache.get(&"B".to_string()), None);
/// ```
pub trait LRUCache<K, V> {
    /// Crée un nouveau cache.
    fn new(capacity: usize) -> Self;

    /// Récupère une valeur du cache.
    /// Met à jour l'ordre si la clé existe.
    fn get(&mut self, key: &K) -> Option<V>;

    /// Ajoute ou met à jour une valeur.
    /// Évince le moins récent si plein.
    fn put(&mut self, key: K, value: V);
}
