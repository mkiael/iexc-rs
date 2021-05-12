mod http;

pub struct Client {
    http_client: http::Client,
    api_token: String,
}

impl Client {
    pub fn new(api_token: String) -> Self {
        Self {
            http_client: http::Client::new("cloud.iexapis.com".to_string()),
            api_token,
        }
    }

    pub fn new_sandbox(api_token: String) -> Self {
        Self {
            http_client: http::Client::new("https://sandbox.iexapis.com".to_string()),
            api_token,
        }
    }
}
