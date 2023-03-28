use std::{
    io,
    net::{TcpListener, TcpStream},
};

use super::{request, Context, HttpResponse, ThreadPool};

pub struct Server {
    thread_pool: ThreadPool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            thread_pool: ThreadPool::new(300),
        }
    }

    fn handle_request(stream: &mut TcpStream) -> io::Result<()> {
        let req = request::parse_request(stream);
        if req.is_none() {
            return Ok(());
        }

        let req = req.unwrap();
        let res = HttpResponse::default();
        let mut ctx = Context::new(req, res, stream);
        ctx.write_text("Hello, world!\r\n")?;
        Ok(())
    }

    pub fn run(&mut self, addr: &str) {
        let listner = TcpListener::bind(addr).unwrap();
        for stream in listner.incoming() {
            match stream {
                Ok(mut stream) => {
                    self.thread_pool.execute(move || {
                        let _ = Self::handle_request(&mut stream).unwrap_or_else(|err| {
                            eprintln!("{}", err);
                        });
                    });
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
    }
}
