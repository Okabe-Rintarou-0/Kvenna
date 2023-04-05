#![allow(dead_code)]

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Arc,
};

use rand::Rng;

use super::{
    skipnode::{KeyType, ValueType},
    SkipNode,
};

enum SearchResult {
    InsertPath(Vec<NonNull<SkipNode>>),
    Exists(NonNull<SkipNode>),
}

pub struct SkipList {
    pub(super) level_heads: Vec<NonNull<SkipNode>>,
    size: usize,
    empty_key: KeyType,
    empty_value: ValueType,
}

unsafe impl Send for SkipList {}

impl SkipList {
    fn top_level(&self) -> NonNull<SkipNode> {
        self.level_heads.last().copied().unwrap()
    }

    fn grow_up(&mut self, path: Vec<NonNull<SkipNode>>, key: KeyType, value: ValueType) {
        let mut rng = rand::thread_rng();
        let mut cur;
        let mut last = None;
        let mut insert_up = true;
        let levels = self.levels();
        let mut idx = levels as isize - 1;
        while insert_up && idx >= 0 {
            let mut left = path[idx as usize];
            cur = SkipNode::new(key.clone(), value.clone());
            unsafe {
                left.as_mut().instert_right(cur);
                cur.as_mut().down = last;
            }
            last = Some(cur);

            idx -= 1;
            insert_up = rng.gen::<f64>() <= 0.5;
        }

        if insert_up {
            let mut sentinel = self.sentinel();
            let down_sentinel = self.top_level();
            cur = SkipNode::new(key, value);
            unsafe {
                cur.as_mut().down = last;
                sentinel.as_mut().down = Some(down_sentinel);
                sentinel.as_mut().instert_right(cur)
            }

            self.level_heads.push(sentinel);
        }
    }

    fn search(&self, key: &str) -> SearchResult {
        let mut path = vec![];
        let mut p = self.top_level();
        for _ in 0..self.levels() {
            while let Some(next) = unsafe { p.as_ref().next } {
                let node_key = unsafe { next.as_ref().key.borrow() };
                let node_key: &str = node_key.deref();
                if node_key > key {
                    break;
                } else if node_key == key {
                    return SearchResult::Exists(next);
                }
                p = next;
            }
            path.push(p);

            if let Some(down) = unsafe { p.as_ref() }.down {
                p = down;
            }
        }
        SearchResult::InsertPath(path)
    }
}

impl SkipList {
    fn sentinel(&self) -> NonNull<SkipNode> {
        SkipNode::new(self.empty_key.clone(), self.empty_value.clone())
    }

    pub fn new() -> Self {
        let mut s = Self {
            level_heads: vec![],
            size: 0,
            empty_key: Arc::new(RefCell::new(String::new())),
            empty_value: Arc::new(RefCell::new(Some(Vec::new()))),
        };
        s.level_heads.push(s.sentinel());
        s
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn levels(&self) -> usize {
        self.level_heads.len()
    }

    fn update(mut node: NonNull<SkipNode>, new_val: Vec<u8>) {
        let mut value_ref = unsafe { node.as_mut().value.borrow_mut() };
        if let Some(value) = value_ref.deref_mut() {
            value.clear();
            value.extend_from_slice(&new_val);
        }
    }

    pub fn put(&mut self, key: &str, value: &[u8]) {
        let result = self.search(&key);
        match result {
            SearchResult::InsertPath(path) => self.grow_up(
                path,
                Arc::new(RefCell::new(key.to_string())),
                Arc::new(RefCell::new(Some(value.to_vec()))),
            ),
            SearchResult::Exists(node) => Self::update(node, value.to_vec()),
        }
    }

    pub fn put_string(&mut self, key: &str, value: &str) {
        self.put(key, value.as_bytes());
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        match self.search(&key) {
            SearchResult::InsertPath(_) => None,
            SearchResult::Exists(node) => {
                let value = unsafe { node.as_ref().value.borrow().clone() };
                value
            }
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key)
            .map(|val| String::from_utf8_lossy(&val).to_string())
    }

    pub fn del(&self, key: &str) -> Option<Vec<u8>> {
        match self.search(&key) {
            SearchResult::InsertPath(_) => None,
            SearchResult::Exists(node) => {
                let value = unsafe { node.as_ref().value.borrow().clone() };
                let mut p = Some(node);
                while let Some(mut node) = p {
                    unsafe {
                        node.as_mut().value.borrow_mut().take();
                        p = node.as_ref().down
                    }
                }
                value
            }
        }
    }
}

impl Drop for SkipList {
    fn drop(&mut self) {
        for head in self.level_heads.iter() {
            let mut head = unsafe { Box::from_raw(head.as_ptr()) };
            loop {
                match head.next {
                    None => break,
                    Some(next) => head = unsafe { Box::from_raw(next.as_ptr()) },
                }
            }
        }
    }
}
