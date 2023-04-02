#![allow(dead_code)]

use std::collections::HashMap;

use super::{
    errors::{self, ServerError},
    method::Method,
    Context,
};

pub type HandleResult = errors::Result<()>;
pub type HandlerFunc = dyn Fn(&mut Context) -> HandleResult + Send + 'static;
pub type BoxedHandlerFunc = Box<HandlerFunc>;

pub struct Router {
    route_map: HashMap<String, BoxedHandlerFunc>,
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
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        let key = Self::url_with_method(url, method);
        self.route_map.insert(key, Box::new(handler));
        self
    }

    pub fn bind_get<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Get, handler)
    }

    pub fn bind_put<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Put, handler)
    }

    pub fn bind_post<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Post, handler)
    }

    pub fn bind_delete<F>(&mut self, url: &str, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Delete, handler)
    }

    pub fn route(
        &mut self,
        url: &str,
        method: Method,
        context: &mut Context,
    ) -> Result<(), ServerError> {
        let key = Self::url_with_method(url, method);
        match self.route_map.get_mut(&key) {
            Some(handler) => {
                handler(context)?;
                Ok(())
            }
            None => Err(ServerError::RouteError(url.to_string())),
        }
    }
}
