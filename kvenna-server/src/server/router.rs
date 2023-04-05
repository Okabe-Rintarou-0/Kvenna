#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

use super::{
    errors::{self, ServerError},
    method::Method,
    request::{ParamsMap, Url},
    status, Context,
};

pub type HandleResult = errors::Result<()>;
pub type HandlerFunc = dyn Fn(&mut Context) -> HandleResult + Send + 'static;
pub type BoxedHandlerFunc = Box<HandlerFunc>;

type RouterMap = HashMap<String, WrappedLink>;
type Link = NonNull<RouterNode>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WrappedLink(Link);

impl WrappedLink {
    fn new(link: Link) -> Self {
        Self(link)
    }
}

unsafe impl Send for WrappedLink {}

impl Deref for WrappedLink {
    type Target = RouterNode;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for WrappedLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

struct RouterNode {
    pub part: String,
    pub next_node_map: RouterMap,
    pub handler: Option<BoxedHandlerFunc>,
    pub is_param: bool,
}

impl RouterNode {
    fn new(part: &str) -> Self {
        let is_param = part.starts_with(':');
        let part = if is_param {
            part[1..].to_owned()
        } else {
            part.to_owned()
        };
        Self {
            part,
            next_node_map: HashMap::new(),
            handler: None,
            is_param,
        }
    }

    fn bind_handler(&mut self, handler: BoxedHandlerFunc) {
        self.handler = Some(handler);
    }

    fn add_router_link(&mut self, next_part: &str, link: WrappedLink) {
        self.next_node_map.insert(next_part.to_string(), link);
    }

    fn handle(&self, ctx: &mut Context) -> errors::Result<()> {
        (self.handler.as_ref().unwrap())(ctx)
    }

    fn get_all_param_links(&self) -> Vec<WrappedLink> {
        let param_links: Vec<_> = self
            .next_node_map
            .values()
            .into_iter()
            .filter(|l| l.is_param)
            .map(|l| *l)
            .collect();
        param_links
    }

    fn get_route_link(&self, next_part: &str) -> Option<WrappedLink> {
        self.next_node_map.get(next_part).copied()
    }

    // wrap will take the ownership of a router node and return the wrapped NonNull<RouterNode>
    fn wrap(self) -> WrappedLink {
        WrappedLink::new(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(self))) })
    }
}

impl Default for RouterNode {
    fn default() -> Self {
        Self {
            part: String::new(),
            next_node_map: HashMap::new(),
            handler: None,
            is_param: false,
        }
    }
}

pub struct Router {
    root: WrappedLink,
}

impl Router {
    fn add_method_nodes(root: &mut RouterNode) {
        for method in vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter() {
            let node = RouterNode::new(method.into());
            root.add_router_link(method.into(), node.wrap());
        }
    }

    pub fn new() -> Self {
        let mut root = RouterNode::default();
        Self::add_method_nodes(&mut root);
        Router { root: root.wrap() }
    }

    fn get_parts(url: &str) -> Vec<&str> {
        let mut parts: Vec<_> = url.split('/').collect();
        // remove the last redundant one
        if let Some(url) = parts.last() {
            if url.is_empty() {
                parts.remove(parts.len() - 1);
            }
        }
        parts
    }

    fn url_with_method(url: &str, method: Method) -> String {
        let url = if url.starts_with('/') { &url[1..] } else { url };
        let method: &str = method.into();
        format!("{}/{}", method, url)
    }

    fn _search_route_node(
        from: WrappedLink,
        parts: &[&str],
        create: bool,
        params: Option<Rc<RefCell<ParamsMap>>>,
    ) -> Option<WrappedLink> {
        let mut cur_node = from;
        for (i, part) in parts.iter().enumerate() {
            // check static link
            if let Some(node) = cur_node.get_route_link(part) {
                cur_node = node;
                continue;
            }
            // check param links
            for next in cur_node.get_all_param_links() {
                let result =
                    Self::_search_route_node(next, &parts[i + 1..], create, params.clone());
                if result.is_some() {
                    if let Some(params) = params {
                        params
                            .as_ref()
                            .borrow_mut()
                            .insert(next.part.clone(), part.to_string());
                    }
                    return result;
                }
            }
            // if don't create new node, return
            if !create {
                return None;
            }
            let new_node = RouterNode::new(&part).wrap();
            cur_node.add_router_link(part, new_node);
            cur_node = new_node;
        }
        if create || cur_node.handler.is_some() {
            Some(cur_node)
        } else {
            None
        }
    }

    fn search_and_create_route_node(&self, parts: &[&str]) -> Option<WrappedLink> {
        Self::_search_route_node(self.root, parts, true, None)
    }

    fn search_route_node(&self, parts: &[&str]) -> Option<WrappedLink> {
        Self::_search_route_node(self.root, parts, false, None)
    }

    fn search_route_node_with_params(&self, parts: &[&str]) -> (Option<WrappedLink>, ParamsMap) {
        let rc = Rc::new(RefCell::new(ParamsMap::new()));
        (
            Self::_search_route_node(self.root, parts, false, Some(rc.clone())),
            rc.as_ref().to_owned().into_inner(),
        )
    }

    fn bind<F>(&mut self, url: &Url, method: Method, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        let url_with_method = Self::url_with_method(url.get_raw(), method);
        let parts = Self::get_parts(&url_with_method);
        let mut node = self.search_and_create_route_node(&parts).unwrap();
        node.bind_handler(Box::new(handler));
        self
    }

    pub fn bind_get<F>(&mut self, url: &Url, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Get, handler)
    }

    pub fn bind_put<F>(&mut self, url: &Url, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Put, handler)
    }

    pub fn bind_post<F>(&mut self, url: &Url, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Post, handler)
    }

    pub fn bind_delete<F>(&mut self, url: &Url, handler: F) -> &mut Self
    where
        F: Fn(&mut Context) -> HandleResult + Send + 'static,
    {
        self.bind(url, Method::Delete, handler)
    }

    pub fn route(
        &mut self,
        url: &Url,
        method: Method,
        ctx: &mut Context,
    ) -> Result<(), ServerError> {
        let url_with_method = Self::url_with_method(url.get_raw(), method);
        let parts = Self::get_parts(&url_with_method);
        let (node, params) = self.search_route_node_with_params(&parts);
        let result = match node {
            Some(node) => {
                ctx.req.url.set_params(params);
                node.handle(ctx)
            }
            None => {
                // set the status code to 404 NOT FOUND
                ctx.status(status::NOT_FOUND);
                Err(ServerError::RouteError(url.get_raw().to_owned()))
            }
        };
        // we should at least return the response headers.
        if !ctx.has_written {
            ctx.write_empty()?;
        }
        result
    }
}
