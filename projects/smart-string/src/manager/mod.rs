use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use ahash::{AHasher, RandomState};
use dashmap::DashMap;
use dashmap::mapref::one::Ref;

pub static STRING_MANAGER: LazyLock<StringManager> = LazyLock::new(|| StringManager::default());

pub type StringID = u64;

pub struct StringManager {
    cache: DashMap<StringID, String, RandomState>,
}

impl Default for StringManager {
    fn default() -> Self {
        let hasher = RandomState::default();
        Self {
            cache: DashMap::with_hasher(hasher),
        }
    }
}

impl StringManager {
    pub fn get(&self, key: StringID) -> Option<Ref<StringID, String, RandomState>> {
        self.cache.get(&key)
    }
    pub fn insert<S>(&self, value: S) -> StringID where S: Into<String> {
        let mut hasher = AHasher::default();
        let s = value.into();
        s.hash(&mut hasher);
        let hash = hasher.finish();
        if self.cache.contains_key(&hash) {
            return hash;
        }
        self.cache.insert(hash, s);
        hash
    }
    pub fn remove(&mut self, key: StringID) -> Option<String> {
        self.cache.remove(&key).map(|v| v.1)
    }
}