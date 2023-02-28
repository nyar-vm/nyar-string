use std::sync::LazyLock;

use dashmap::DashMap;

pub static STRING_MANAGER: LazyLock<StringManager> = LazyLock::new(|| StringManager::default());

pub type StringID = usize;

pub struct StringManager {
    cache: DashMap<usize, String>,
}

impl Default for StringManager {
    fn default() -> Self {
        Self {
            cache: DashMap::with_hasher(),
        }
    }
}

impl StringManager {
    pub fn get(&self, key: StringID) -> Option<&str> {
        self.cache.get(&key).map(|v| v.value().as_str())
    }
    pub fn set<S: Into<String>>(&self, key: StringID, value: String) -> StringID {
        self.cache.insert(key, value);
    }
    pub fn insert<S: Into<String>>(&self, value: String) -> StringID {
        let key = self.cache.len();
        self.cache.insert(key, value);
        key
    }
}