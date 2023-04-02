mod kvenna;
mod server;
mod skiplist;

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    io,
    sync::{Arc, Mutex},
};

use argparse::{ArgumentParser, Store, StoreTrue};
use skiplist::helper;

use crate::{kvenna::Kvenna, server::Server, skiplist::SkipList};

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
                    skiplist.put_string(parts[1], parts[2]);
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
    pub host: String,
    pub port: u16,
}

fn main() {
    let mut opt = Options {
        interactive: false,
        host: "127.0.0.1".to_string(),
        port: 5000,
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Kevanna server");
        ap.refer(&mut opt.interactive).add_option(
            &["-i", "--interactive"],
            StoreTrue,
            "Interactive mode",
        );
        ap.refer(&mut opt.host)
            .add_option(&["-h", "--host"], Store, "Server host");
        ap.refer(&mut opt.port)
            .add_option(&["-p", "--port"], Store, "Server port");
        ap.parse_args_or_exit();
    }

    if opt.interactive {
        loop {
            interact(&mut SkipList::new());
        }
    }

    let addr = format!("{}:{}", opt.host, opt.port);
    println!("Server is running on {}", addr);
    let kv_store = Arc::new(Mutex::new(Kvenna::new()));
    let mut server = Server::new();
    let cloned = kv_store.clone();
    server.bind_get("/", move |c| {
        c.write_text(&cloned.lock().unwrap().get_string("test").unwrap());
    });
    let cloned = kv_store.clone();
    server.bind_put("/", move |c| {
        kv_store.lock().unwrap().put_string("test", "test_value");
        c.write_text("ok");
    });
    server.run(&addr);
}
