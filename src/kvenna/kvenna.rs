#![allow(dead_code)]

use crate::skiplist::SkipList;

pub struct Kvenna {
    skiplist: SkipList,
}

impl Kvenna {
    pub fn new() -> Self {
        Self {
            skiplist: SkipList::new(),
        }
    }

    pub fn put(&mut self, key: &str, value: &[u8]) {
        self.skiplist.put(key, value);
    }

    pub fn put_string(&mut self, key: &str, value: &str) {
        self.put(key, value.as_bytes());
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.skiplist.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)
            .map(|val| String::from_utf8_lossy(&val).to_string())
    }

    pub fn del(&self, key: &str) -> Option<Vec<u8>> {
        self.skiplist.del(key)
    }
}
