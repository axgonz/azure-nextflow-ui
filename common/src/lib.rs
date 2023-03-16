pub mod types;
pub use types::*; 

// use leptos::*;

pub use reqwest::{
    Response,
    Error,
};

pub struct WebHelpers {}

impl WebHelpers {
    pub async fn web_get(uri: &String) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        match client.get(uri)
            .send()
            .await {
            Ok(response) => {
                // log!("[reqwest] GET {:#?}...Ok", uri);
                return Ok(response)
            }
            Err(error) => {
                // log!("[reqwest] GET {:#?}...Err", uri);
                return Err(error)
            }
        };
    }   

    pub async fn web_post(uri: &String, json: &Value) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        match client.post(uri)
            .json(json)
            .send()
            .await {
            Ok(response) => {
                // log!("[reqwest] POST {:#?}...Ok", uri);
                return Ok(response)
            }
            Err(error) => {
                // log!("[reqwest] POST {:#?}...Err", uri);
                return Err(error)
            }
        };
    }   
}
