#![allow(dead_code)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

struct Cache<V> {
    storage: HashMap<String, (V, Instant)>,
}

impl<V: Clone> Cache<V> {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: V, ttl: Duration) {
        self.storage.insert(key, (value, Instant::now() + ttl));
    }

    fn get(&self, key: &str) -> Option<V> {
        self.storage.get(key).and_then(|(value, expiration)| {
            if Instant::now() < *expiration {
                Some(value.clone())
            } else {
                None
            }
        })
    }

    fn get_or_set(&mut self, key: String, value: V, ttl: Duration) -> V {
        if let Some(existing_value) = self.get(&key) {
            existing_value.clone()
        } else {
            self.set(key, value.clone(), ttl);
            value
        }
    }

    fn expire(&mut self, key: &str) -> Option<V> {
        let (value, ttl) = self.storage.remove(key)?;
        if Instant::now() < ttl {
            Some(value)
        } else {
            None
        }
    }

    fn expire_all(&mut self) {
        self.storage.retain(|_, (_, ttl)| Instant::now() < *ttl);
    }

    fn refresh(&mut self, key: &str, ttl: Duration) -> bool {
        if let Some((value, _)) = self.storage.get(key) {
            let value = value.clone();
            self.set(key.to_string(), value, ttl);
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.storage.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut cache = Cache::new();
        cache.set(
            "key".to_owned(),
            "value".to_owned(),
            Duration::from_secs(10),
        );
        assert_eq!(cache.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_get() {
        let mut cache = Cache::new();
        cache.set(
            "key".to_owned(),
            "value".to_owned(),
            Duration::from_secs(10),
        );
        assert_eq!(cache.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_get_or_set() {
        let mut cache = Cache::new();
        let value = cache.get_or_set(
            "key".to_owned(),
            "value".to_owned(),
            Duration::from_secs(10),
        );
        assert_eq!(value, "value".to_string());
    }

    #[test]
    fn test_expire() {
        let mut cache = Cache::new();
        let ttl = Duration::from_secs(2);
        cache.set("key".to_owned(), "value".to_owned(), ttl);
        assert_eq!(cache.get("key"), Some("value".to_string()));
        std::thread::sleep(Duration::from_secs(1));
        assert_eq!(cache.get("key"), Some("value".to_string()));
        std::thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get("key"), None);
    }

    #[test]
    fn test_expire_all() {
        let mut cache = Cache::new();
        let ttl = Duration::from_secs(1);
        cache.set("key".to_owned(), "value".to_owned(), ttl);
        cache.set("key2".to_owned(), "value2".to_owned(), ttl);
        std::thread::sleep(ttl + Duration::from_secs(1));
        cache.expire_all();
        assert_eq!(cache.get("key"), None);
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_refresh() {
        let key = "key".to_owned();
        let mut cache = Cache::new();
        let ttl = Duration::from_secs(10);
        cache.set(key.clone(), "value".to_owned(), ttl);
        std::thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.refresh(&key, Duration::from_secs(10)), true);
        cache.expire(&key);
        assert_eq!(cache.refresh("key", Duration::from_secs(10)), false);
    }

    #[test]
    fn test_clear() {
        let mut cache = Cache::new();
        cache.set(
            "key".to_owned(),
            "value".to_owned(),
            Duration::from_secs(10),
        );
        cache.set(
            "key2".to_owned(),
            "value2".to_owned(),
            Duration::from_secs(10),
        );
        cache.clear();
        assert_eq!(cache.get("key"), None);
        assert_eq!(cache.get("key2"), None);
    }
}
