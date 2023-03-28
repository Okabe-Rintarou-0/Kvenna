#![allow(dead_code)]

use std::{
    io::{self, Write},
    net::TcpStream,
};

use super::{headers::Headers, status, version::Version};

pub struct HttpResponse {
    pub version: Version,
    pub status_code: u32,
    pub status_text: String,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: Version::V11,
            status_code: 200,
            status_text: "OK".to_string(),
            headers: Headers::new(),
            body: None,
        }
    }
}

impl HttpResponse {
    pub fn new(
        version: Version,
        status_code: u32,
        status_text: String,
        headers: Headers,
        body: Option<Vec<u8>>,
    ) -> Self {
        Self {
            version,
            status_code,
            status_text,
            headers,
            body,
        }
    }

    fn get_status_text(status_code: u32) -> String {
        match status_code {
            status::STATUS_OK => "OK".to_string(),
            status::STATUS_BAD_REQUEST => "Bad Request".to_string(),
            status::STATUS_UNAUTHORIZED => "Unauthorized".to_string(),
            status::STATUS_FORBIDDEN => "Forbidden".to_string(),
            _ => "Not Found".to_string(),
        }
    }

    pub fn status(&mut self, status_code: u32) -> &mut Self {
        self.status_code = status_code;
        self.status_text = Self::get_status_text(status_code);
        self
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.put(key, value);
        self
    }

    pub fn body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = Some(body);
        self
    }
}

pub(crate) fn write_response(stream: &mut TcpStream, res: &HttpResponse) -> io::Result<()> {
    let version: String = res.version.into();
    let req_line = format!("{} {} {}\r\n", version, res.status_code, res.status_text);
    stream.write(req_line.as_bytes())?;
    for (key, value) in res.headers.iter() {
        stream.write(format!("{}: {}\r\n", key, value).as_bytes())?;
    }
    stream.write("\r\n".as_bytes())?;
    if let Some(ref body) = res.body {
        stream.write(body)?;
    }
    Ok(())
}
