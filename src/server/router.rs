#![allow(dead_code)]

use std::{collections::HashMap, error, fmt};

use super::{method::Method, Context};

pub struct Router {
    route_map: HashMap<String, Box<dyn Fn(&mut Context) + Send + 'static>>,
}

#[derive(Debug, Clone)]
pub struct RouteError {
    url: String,
}

impl RouteError {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "route url {} is not configured", self.url)
    }
}

impl error::Error for RouteError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            route_map: HashMap::new(),
        }
    }

    fn url_with_method(url: &str, method: Method) -> String {
        let method: &str = method.into();
        format!("{}_{}", method, url)
    }

    pub fn bind<F>(&mut self, url: &str, method: Method, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) + Send + 'static,
    {
        let key = Self::url_with_method(url, method);
        self.route_map.insert(key, Box::new(handler));
        self
    }

    pub fn bind_get<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) + Send + 'static,
    {
        self.bind(url, Method::Get, handler)
    }

    pub fn bind_put<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) + Send + 'static,
    {
        self.bind(url, Method::Put, handler)
    }

    pub fn bind_post<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) + Send + 'static,
    {
        self.bind(url, Method::Post, handler)
    }

    pub fn bind_delete<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) + Send + 'static,
    {
        self.bind(url, Method::Delete, handler)
    }

    pub fn route(
        &mut self,
        url: &str,
        method: Method,
        context: &mut Context,
    ) -> Result<(), RouteError> {
        let key = Self::url_with_method(url, method);
        match self.route_map.get_mut(&key) {
            Some(handler) => {
                handler(context);
                Ok(())
            }
            None => Err(RouteError::new(url)),
        }
    }
}
