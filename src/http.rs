//! This module is an adapter for HTTP related operations.
//! In the future, if we want to migrate to a different HTTP library,
//! we can easily change this facade instead of multiple places in the crate.

use reqwest::{blocking::Client, IntoUrl};
use std::env;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error(transparent)]
#[diagnostic(code("fnm::http::error"))]
pub struct Error(#[from] reqwest::Error);
pub type Response = reqwest::blocking::Response;

pub fn get(url: impl IntoUrl) -> Result<Response, Error> {
    let mut client_builder = Client::builder();
    
    // Check if proxy environment variable exists and apply it
    if let Ok(proxy_url) = env::var("FNM_PROXY") {
        if !proxy_url.is_empty() {
            // Create a proxy from the environment variable
            match reqwest::Proxy::all(&proxy_url) {
                Ok(proxy) => {
                    client_builder = client_builder.proxy(proxy);
                },
                Err(e) => {
                    // Log error but continue without proxy
                    eprintln!("Failed to set proxy from FNM_PROXY environment variable: {}", e);
                }
            }
        }
    }

    let client = client_builder.build()?;
    
    Ok(client
        .get(url)
        // Some sites require a user agent.
        .header("User-Agent", concat!("fnm ", env!("CARGO_PKG_VERSION")))
        .send()?)
}