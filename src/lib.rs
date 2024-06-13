#[macro_use]
extern crate tracing;

use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

pub mod analyst_estimate;
pub mod company;
pub mod earning;
pub mod financial;
pub mod forex;
pub mod historical_price;
pub mod news;
pub mod period;
pub mod quote;
pub mod stock;

pub struct Client {
    pub base: String,
    pub api_key: String,
}

impl Client {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        Client {
            base: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[cfg(debug_assertions)]
async fn decode_content<T>(response: Response) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let content = match response.text().await {
        Ok(content) => content,
        Err(e) => {
            error!("{:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    serde_json::from_str::<T>(&content).map_err(|e| {
        error!("unable to deserialize: {e:?}\npayload: {content}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[cfg(not(debug_assertions))]
async fn decode_content<T>(response: Response) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    response.json::<T>().await.map_err(|e| {
        error!("{:?}", e);
        StatusCode::BAD_REQUEST
    })
}

async fn request<T>(endpoint: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    match reqwest::get(endpoint).await {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
            decode_content(r).await
        }
        Err(e) => {
            if e.is_status() {
                Err(e.status().unwrap())
            } else {
                error!("{:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
