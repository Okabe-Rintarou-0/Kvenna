use argparse::{ArgumentParser, Store};
use client::Client;
use dialoguer::Input;

mod client;

struct Options {
    pub host: String,
    pub port: u16,
}

async fn hanlde_get_cmd(cli: &Client, args: &[&str]) {
    if args.len() != 1 {
        println!("Usage: get <key>");
    } else {
        let key = args[0];
        match cli.get_string(key).await {
            Some(val) => {
                println!("{}", val);
            }
            None => println!("Key {} not found", key),
        }
    }
}

async fn hanlde_put_cmd(cli: &Client, args: &[&str]) {
    if args.len() != 2 {
        println!("Usage: put <key> <value>");
    } else {
        let (key, value) = (args[0], args[1]);
        match cli.put_string(key, value).await {
            Some(val) => {
                println!("{}", val);
            }
            None => println!("Something wrong, please retry!"),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut opt = Options {
        host: "127.0.0.1".to_string(),
        port: 5000,
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Kevanna client");
        ap.refer(&mut opt.host)
            .add_option(&["-h", "--host"], Store, "Server host");
        ap.refer(&mut opt.port)
            .add_option(&["-p", "--port"], Store, "Server port");
        ap.parse_args_or_exit();
    }
    let addr = format!("http://{}:{}", opt.host, opt.port);
    println!("Specified host: {:?}", addr);

    let cli = Client::new(addr);
    loop {
        let input: String = Input::new()
            .with_prompt("Kvenna cli")
            .interact_text()
            .unwrap();
        let args: Vec<_> = input.split_whitespace().collect();
        if !args.is_empty() {
            let op = args[0].to_lowercase();
            match op.as_str() {
                "get" => hanlde_get_cmd(&cli, &args[1..]).await,
                "put" => hanlde_put_cmd(&cli, &args[1..]).await,
                _ => println!("Unknown command"),
            }
        }
    }
}
