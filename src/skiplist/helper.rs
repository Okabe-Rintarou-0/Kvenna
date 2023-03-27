use std::ptr::NonNull;

use super::{SkipList, SkipNode};

fn traverse_level(mut head: NonNull<SkipNode>) {
    print!("sentinel");
    loop {
        let next = unsafe { head.as_ref().next };
        match next {
            None => {
                println!("");
                return;
            }
            Some(next) => {
                let x = unsafe { next.as_ref().key.borrow_mut() };
                print!(" -> {}", x);
                head = next;
            }
        }
    }
}

pub fn display(skiplist: &SkipList) {
    let levels = skiplist.levels();
    for level in (0..levels).rev() {
        let head = skiplist.level_heads[level];
        print!("level {}: ", level + 1);
        traverse_level(head);
    }
}
