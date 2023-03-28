#[derive(Debug, PartialEq, Eq)]
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
            "Get" => Method::Get,
            "Post" => Method::Post,
            "Put" => Method::Put,
            "Delete" => Method::Delete,
            _ => Method::Unsupported,
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
