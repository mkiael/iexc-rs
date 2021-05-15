mod http;

pub enum Endpoint {
    Production,
    Sandbox,
}

pub struct Client {
    http_client: http::Client,
    api_token: String,
}

impl Client {
    pub fn new(endpoint: Endpoint, api_token: String) -> Self {
        let domain = match endpoint {
            Endpoint::Production => "cloud.iexapis.com",
            Endpoint::Sandbox => "sandbox.iexapis.com",
        };

        Self {
            http_client: http::Client::new(domain.to_string()),
            api_token,
        }
    }
}
