#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Unsupported,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Unsupported,
        }
    }
}

impl Into<&str> for Method {
    fn into(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Unsupported => "UNSUPPORTED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        let get: Method = "Get".into();
        assert_eq!(get, Method::Get);
    }
}
