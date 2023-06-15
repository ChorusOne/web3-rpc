use crate::model::JsonRpcError;
use anyhow::bail;
use reqwest::{Response, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Client {
    pub url: String,
    pub client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    pub fn default_client() -> reqwest_middleware::ClientWithMiddleware {
        let wrapped_client = reqwest::Client::new();
        reqwest_middleware::ClientBuilder::new(wrapped_client).build()
    }

    pub fn new(url: String) -> Self {
        Self::new_with_client(url, Self::default_client())
    }

    pub fn new_with_client(url: String, client: reqwest_middleware::ClientWithMiddleware) -> Self {
        Client { url, client }
    }

    pub async fn post(&self, payload: Value) -> anyhow::Result<String> {
        let response = self.client.post(&self.url).json(&payload).send().await?;

        self.handler(response).await
    }

    async fn handler(&self, response: Response) -> anyhow::Result<String> {
        match response.status() {
            StatusCode::OK => {
                let text = response.text().await?.as_str().to_string();
                if text.find("error") != None {
                    let e: JsonRpcError<HashMap<String, String>> =
                        serde_json::from_str(text.as_str())?;
                    bail!(e.error.message)
                }
                return Ok(text);
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                bail!(format!("Bad Request: {:?}", response));
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        };
    }
}
