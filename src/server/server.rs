use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use super::{request, router::HandleResult, Context, HttpResponse, Router, ThreadPool, errors};

pub struct Server {
    thread_pool: ThreadPool,
    pub router: Arc<Mutex<Router>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            thread_pool: ThreadPool::new(300),
            router: Arc::new(Mutex::new(Router::new())),
        }
    }

    pub fn bind_get<F>(&mut self, url: &str, handler: F)
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.router.lock().unwrap().bind_get(url, handler);
    }

    pub fn bind_put<F>(&mut self, url: &str, handler: F)
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.router.lock().unwrap().bind_put(url, handler);
    }

    fn handle_request(router: Arc<Mutex<Router>>, stream: &mut TcpStream) -> errors::Result<()> {
        let req = request::parse_request(stream)?;
        let res = HttpResponse::default();
        let (url, method) = (req.url.get_raw(), req.method);
        let mut ctx = Context::new(req, res, stream);
        router.lock().unwrap().route(&url, method, &mut ctx)?;
        let method: &str = method.into();
        println!("{} {}", method, url);
        Ok(())
    }

    pub fn run(&mut self, addr: &str) {
        let listner = TcpListener::bind(addr).unwrap();
        for stream in listner.incoming() {
            match stream {
                Ok(mut stream) => {
                    let router = self.router.clone();
                    self.thread_pool.execute(move || {
                        let _ = Self::handle_request(router, &mut stream).unwrap_or_else(|err| {
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
