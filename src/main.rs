mod skiplist;

use argparse::{ArgumentParser, StoreTrue};
use skiplist::helper;
use std::io;

use crate::skiplist::SkipList;

fn print_value(value: Option<Vec<u8>>) {
    match value {
        Some(value) => {
            println!("{}", String::from_utf8(value).unwrap());
        }
        None => println!("None"),
    }
}

fn interact(skiplist: &mut SkipList) {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let parts: Vec<_> = input.split_whitespace().collect();
    let n = parts.len();
    if n >= 1 {
        let op = parts[0];
        match op {
            "display" => helper::display(skiplist),
            "get" => {
                if n == 2 {
                    print_value(skiplist.get(parts[1]));
                }
            }
            "put" => {
                if n == 3 {
                    skiplist.put(parts[1].to_string(), parts[2].into());
                    println!("ok");
                }
            }
            "del" => {
                if n == 2 {
                    print_value(skiplist.del(parts[1]));
                }
            }
            _ => {
                println!("invalid operation");
            }
        }
    }
}

struct Options {
    pub interactive: bool,
}

fn main() {
    let mut opt = Options { interactive: false };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Kevanna server");
        ap.refer(&mut opt.interactive).add_option(
            &["-i", "--interactive"],
            StoreTrue,
            "Interactive mode",
        );
        ap.parse_args_or_exit();
    }

    let mut skip_list = SkipList::new();
    if opt.interactive {
        loop {
            interact(&mut skip_list);
        }
    }
}
