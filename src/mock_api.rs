//! # API simulée pour les tests
//! 
//! Simule une API distante avec un délai configurable.
//! Retourne des données au format `data_X` pour une clé `X`.
//! 
//! ## Exemple
//! ```no_run
//! use lru_cache::mock_api::MockApi;
//! use std::time::Duration;
//! 
//! let api = MockApi::new(Duration::from_millis(100));
//! assert_eq!(api.get_data("A"), "data_A".to_string());
//! ```

use std::time::Duration;
use std::thread;

/// API simulée pour les tests.
/// 
/// Simule une API distante avec un délai configurable.
/// Retourne des données au format `data_X` pour une clé `X`.
/// 
/// # Exemple
/// ```no_run
/// use lru_cache::mock_api::MockApi;
/// use std::time::Duration;
/// 
/// let api = MockApi::new(Duration::from_millis(100));
/// assert_eq!(api.get_data("A"), "data_A".to_string());
/// ```
pub struct MockApi {
    delay: Duration,
}

impl MockApi {
    /// Crée une nouvelle API mock avec un délai spécifié.
    pub fn new(delay: Duration) -> Self {
        MockApi { delay }
    }

    /// Simule une requête API avec un délai.
    /// 
    /// Retourne `data_X` pour une clé `X` après le délai configuré.
    pub fn get_data(&self, key: &str) -> String {
        thread::sleep(self.delay);
        format!("data_{}", key)
    }
}
