#![allow(dead_code)]

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use super::{
    errors::{self, ServerError},
    headers::{self, Headers},
    method::Method,
    version::Version,
};

pub type ParamsMap = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Url {
    raw: String,
    params: Option<ParamsMap>,
}

impl Url {
    pub fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_owned(),
            params: None,
        }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn set_params(&mut self, params: ParamsMap) {
        self.params = Some(params);
    }

    pub fn get_param(&self, name: &str) -> Option<&str> {
        match self.params {
            Some(ref params) => params.get(name).map(|s| s.as_str()),
            None => None,
        }
    }
}

impl Default for Url {
    fn default() -> Self {
        Self {
            raw: String::new(),
            params: None,
        }
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

pub(super) fn parse_req_line(req_line: String) -> errors::Result<(Method, Url, Version)> {
    let parts: Vec<_> = req_line.split_whitespace().collect();
    if parts.len() == 3 {
        let method: Method = parts[0].into();
        let url = Url::new(parts[1]);
        let version: Version = parts[2].into();
        Ok((method, url, version))
    } else {
        Err(ServerError::PareRequestError)
    }
}

fn read_req_line(buf_reader: &mut BufReader<&mut TcpStream>) -> errors::Result<HttpRequest> {
    let mut req = HttpRequest::default();
    let mut line = String::new();
    let num_bytes = buf_reader.read_line(&mut line)?;
    if num_bytes == 0 {
        return Err(ServerError::PareRequestError);
    }
    let (method, url, version) = parse_req_line(line)?;
    req.method = method;
    req.url = url;
    req.version = version;
    Ok(req)
}

fn read_headers(
    buf_reader: &mut BufReader<&mut TcpStream>,
    req: &mut HttpRequest,
) -> errors::Result<()> {
    let headers = req.headers.borrow_mut();
    let mut line = String::new();
    loop {
        let num_bytes = buf_reader.read_line(&mut line)?;
        if num_bytes == 0 || line == "\r\n" {
            break;
        }
        let parts: Vec<_> = line.splitn(2, ": ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            headers.insert(key, value);
        }
        line.clear();
    }
    Ok(())
}

fn read_body(
    buf_reader: &mut BufReader<&mut TcpStream>,
    req: &mut HttpRequest,
) -> errors::Result<()> {
    if let Some(len) = req.headers.get(headers::CONTENT_LENGTH) {
        if let Ok(len) = len.parse::<usize>() {
            req.body = vec![0; len];
            buf_reader.read_exact(&mut req.body)?;
            return Ok(());
        } else {
            return Err(ServerError::PareRequestError);
        }
    }
    Ok(())
}

pub(super) fn parse_request(stream: &mut TcpStream) -> errors::Result<HttpRequest> {
    let mut buf_reader = BufReader::new(stream);
    let mut req = read_req_line(&mut buf_reader)?;
    read_headers(&mut buf_reader, &mut req)?;
    read_body(&mut buf_reader, &mut req)?;
    Ok(req)
}
