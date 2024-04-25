use crate::model::JsonRpcError;
use error_stack::{IntoReportCompat, Report};
use error_stack::{Context, ResultExt};
use reqwest::{Response, StatusCode};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

#[derive(Clone)]
pub struct Client {
    pub url: String,
    pub client: reqwest_middleware::ClientWithMiddleware,
}

#[derive(Debug)]
pub enum Error {
    JsonRpcError(String),
    IoError,
    HttpError(StatusCode),
    UnexpectedResponseFormat,
    FailedToDeserialize,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Context for Error {}

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

    pub async fn post(&self, payload: Value) -> Result<String, Report<Error>> {
        let response = self
        .client
        .post(&self.url)
        .json(&payload)
        .send()
        .await
        // workaround for https://github.com/hashintel/hash/issues/4355
        .map_err(|e| anyhow::anyhow!(e))
        .into_report()
        .map_err(|e|e.change_context(Error::IoError))?;

        self.handler(response).await
    }

    async fn handler(&self, response: Response) -> Result<String, Report<Error>> {
        match response.status() {
            StatusCode::OK => {
                let text = response
                    .text()
                    .await
                    .change_context(Error::UnexpectedResponseFormat)
                    .attach_printable("response not text")?
                    .as_str()
                    .to_string();
                if text.find("error") != None {
                    let e: JsonRpcError<HashMap<String, String>> =
                        serde_json::from_str(text.as_str())
                            .change_context(Error::UnexpectedResponseFormat)
                            .attach_printable("unexpected err format")?;
                    error_stack::bail!(Error::JsonRpcError(e.error.message))
                }
                return Ok(text);
            }
            s => {
                response
                    .error_for_status()
                    .change_context(Error::HttpError(s))?;
                return Err(error_stack::report!(Error::UnexpectedResponseFormat)
                    .attach_printable(format!("status {}", s)));
            }
        };
    }
}
