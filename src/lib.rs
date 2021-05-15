mod http;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

    pub fn get_latest_price(&self, symbol: &str) -> Result<f64> {
        let resp = self.http_client.get(&format!(
            "/stable/stock/{}/price?token={}",
            symbol, self.api_token
        ))?;

        Ok(resp.body.parse::<f64>()?)
    }
}
