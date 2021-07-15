use std::collections::HashMap;

/// `KvStore` is a simple-to-use, efficient key value store
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// creates a new `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// sets or updates the value in the key value store at the requested key
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut kv = KvStore::new();
    /// kv.set("key".to_owned(), "value".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// `get(key)` retrieves the value in the key value store at the requested key
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut kv = KvStore::new();
    /// kv.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(Some("value".to_owned()), kv.get("key".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// `get(key)` removes the value in the key value store at the requested key
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut kv = KvStore::new();
    /// kv.set("key".to_owned(), "value".to_owned());
    /// kv.remove("key".to_owned());
    /// assert_eq!(None, kv.get("key".to_owned()));
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
