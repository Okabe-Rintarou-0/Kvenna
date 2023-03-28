#![allow(dead_code)]
use std::{borrow::Borrow, io, net::TcpStream};

use super::{headers, response, HttpRequest, HttpResponse};

pub struct Context<'a> {
    pub req: HttpRequest,
    pub res: HttpResponse,
    pub stream: &'a mut TcpStream,
}

impl<'a> Context<'a> {
    pub fn new(req: HttpRequest, res: HttpResponse, stream: &'a mut TcpStream) -> Context<'a> {
        Context { req, res, stream }
    }

    pub fn write_text(&mut self, text: &str) -> io::Result<()> {
        self.res
            .add_header(headers::CONTENT_TYPE, headers::CONTENT_TEXT_HTML)
            .add_header(headers::CONTENT_LENGTH, text.len().to_string().borrow())
            .body(text.as_bytes().to_vec());
        response::write_response(self.stream, &mut self.res)
    }
}
