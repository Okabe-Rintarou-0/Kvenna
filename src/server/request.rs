#![allow(dead_code)]

use std::{
    borrow::{Borrow, BorrowMut},
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use super::{
    headers::{self, Headers},
    method::Method,
    version::Version,
};

#[derive(Debug)]
pub struct Url {
    raw: String,
}

impl Url {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn get_raw(&self) -> &str {
        self.raw.borrow()
    }
}

impl Default for Url {
    fn default() -> Self {
        Self { raw: String::new() }
    }
}

pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
    pub version: Version,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: Method::Unsupported,
            url: Url::default(),
            version: Version::Unsupported,
            headers: Headers::new(),
            body: vec![],
        }
    }
}

pub(super) fn parse_req_line(req_line: String) -> Option<(Method, Url, Version)> {
    let parts: Vec<_> = req_line.split_whitespace().collect();
    if parts.len() == 3 {
        let method: Method = parts[0].into();
        let url = Url::new(parts[1].to_string());
        let version: Version = parts[2].into();
        Some((method, url, version))
    } else {
        None
    }
}

fn read_req_line(buf_reader: &mut BufReader<&mut TcpStream>) -> Option<HttpRequest> {
    let mut req = HttpRequest::default();
    let mut line = String::new();
    let r = buf_reader.read_line(&mut line);
    if r.is_err() || r.unwrap() == 0 {
        return None;
    }
    parse_req_line(line).map(|(method, url, version)| {
        req.method = method;
        req.url = url;
        req.version = version;
        req
    })
}

fn read_headers(buf_reader: &mut BufReader<&mut TcpStream>, req: &mut HttpRequest) {
    let headers = req.headers.borrow_mut();
    let mut line = String::new();
    loop {
        match buf_reader.read_line(&mut line) {
            Ok(n_bytes) => {
                if n_bytes == 0 || line == "\r\n" {
                    break;
                }
            }
            Err(_) => break,
        }
        let parts: Vec<_> = line.splitn(2, ": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            headers.insert(key, value);
        }
        line.clear();
    }
}

fn read_body(buf_reader: &mut BufReader<&mut TcpStream>, req: &mut HttpRequest) {
    if let Some(len) = req.headers.get(headers::CONTENT_LENGTH) {
        if let Ok(len) = len.parse::<usize>() {
            req.body = vec![0; len];
            let _ = buf_reader.read_exact(&mut req.body);
        }
    }
}

pub(super) fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
    let mut buf_reader = BufReader::new(stream);
    if let Some(mut req) = read_req_line(&mut buf_reader) {
        read_headers(&mut buf_reader, &mut req);
        read_body(&mut buf_reader, &mut req);
        Some(req)
    } else {
        None
    }
}
