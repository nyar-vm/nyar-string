use std::{
    hash::{Hash, Hasher},
    sync::LazyLock,
};

use ahash::{AHasher, RandomState};
use dashmap::DashMap;

pub static STRING_MANAGER: LazyLock<StringManager> = LazyLock::new(|| StringManager::default());

pub type StringID = usize;

pub struct StringManager {
    cache: Radix
}

impl Default for StringManager {
    fn default() -> Self {
        let hasher = RandomState::default();
        Self { cache: DashMap::with_hasher(hasher) }
    }
}

impl StringManager {
    /// Get a reference to the string
    pub fn get(&self, key: StringID) -> Option<&str> {
        Some(self.cache.get(&key)?.as_ref())
    }
    pub fn get_hash_key(string: &str) -> StringID {
        let mut hasher = AHasher::default();
        string.hash(&mut hasher);
        hasher.finish() as usize
    }

    pub fn insert(&self, value: String) -> StringID {
        let hash = Self::get_hash_key(&value);
        if self.cache.contains_key(&hash) {
            return hash;
        }
        self.cache.insert(hash, value);
        hash
    }
    pub fn remove(&mut self, key: StringID) -> Option<String> {
        self.cache.remove(&key).map(|v| v.1)
    }
}
