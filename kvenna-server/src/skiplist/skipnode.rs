use std::{cell::RefCell, ptr::NonNull, sync::Arc};

pub(super) type Link = Option<NonNull<SkipNode>>;
pub(super) type KeyType = Arc<RefCell<String>>;
pub(super) type ValueType = Arc<RefCell<Option<Vec<u8>>>>;

#[derive(Eq)]
pub struct SkipNode {
    pub key: KeyType,
    pub value: ValueType,
    pub prev: Link,
    pub next: Link,
    pub down: Link,
}

impl SkipNode {
    pub(super) fn new(key: KeyType, value: ValueType) -> NonNull<SkipNode> {
        let n = Box::new(Self {
            key: key,
            value,
            prev: None,
            next: None,
            down: None,
        });
        let n_ptr = Box::into_raw(n);
        unsafe { NonNull::new_unchecked(n_ptr) }
    }

    pub(super) fn instert_right(&mut self, mut new: NonNull<SkipNode>) {
        if let Some(mut old) = self.next {
            unsafe {
                new.as_mut().next = Some(old);
                old.as_mut().prev = Some(new);
            };
        }
        self.next = Some(new);
        unsafe { new.as_mut().prev = Some(NonNull::new_unchecked(self as *mut SkipNode)) }
    }
}

impl Drop for SkipNode {
    fn drop(&mut self) {
        let key = self.key.borrow();
        if !key.is_empty() {
            println!("Dropping node {}", key);
        } else {
            println!("Dropping sentinel node");
        }
    }
}

impl PartialEq for SkipNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Ord for SkipNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for SkipNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
