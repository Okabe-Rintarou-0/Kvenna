#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Version {
    V11,
    V20,
    Unsupported,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V11,
            "HTTP/2.0" => Version::V20,
            _ => Version::Unsupported,
        }
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        match self {
            Version::V20 => "HTTP/2.0".to_owned(),
            _ => "HTTP/1.1".to_owned(),
        }
    }
}
