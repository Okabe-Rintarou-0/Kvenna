#![allow(dead_code)]

use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub const USER_AGENT: &str = "User-Agent";
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const CONTENT_TEXT_HTML: &str = "text/html; charset=utf-8";

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}

impl Deref for Headers {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        self.0.borrow()
    }
}

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.borrow_mut()
    }
}
