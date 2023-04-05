use reqwest::StatusCode;

pub struct Client {
    api_base_url: String,
}

impl Client {
    pub fn new(api_base_url: String) -> Self {
        Self { api_base_url }
    }

    pub async fn get_string(&self, key: &str) -> Option<String> {
        let api_url = format!("{}/{}", self.api_base_url, key);
        let result = reqwest::get(api_url).await;
        if result.is_err() {
            return None;
        }
        let resp = result.unwrap();
        match resp.status() {
            StatusCode::OK => {
                if let Ok(val) = resp.text().await {
                    Some(val)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub async fn put_string(&self, key: &str, value: &str) -> Option<String> {
        let api_url = format!("{}/{}/{}", self.api_base_url, key, value);
        let cli = reqwest::Client::new();
        let result = cli.put(api_url).send().await;
        if result.is_err() {
            println!("{:?}", result);
            return None;
        }
        let resp = result.unwrap();
        match resp.status() {
            StatusCode::OK => {
                if let Ok(val) = resp.text().await {
                    Some(val)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
