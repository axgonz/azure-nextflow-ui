pub mod types;
pub use types::*; 

use leptos::log;

use std::time::Duration;
use async_std::task;

pub use reqwest::{
    Response,
    Error,
    StatusCode
};

pub struct WebHelpers {}

impl WebHelpers {
    pub async fn web_get_final(uri: &String) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let req = client.get(uri)
            .send();
        
        match req.await {
            Ok(response) => {
                log!("[reqwest] GET {:#?}...Ok", uri);
                return Ok(response)
            }
            Err(error) => {
                log!("[reqwest] GET {:#?}...Err", uri);
                return Err(error)
            }
        };
    }   

    pub async fn web_get(uri: &String, mut retries: u8) -> Result<Response, Error> {
        let mut delay = 4;
        
        while retries > 0 {
            let result = Self::web_get_final(uri).await;
       
            match result {
                Ok(result) => {
                    match result.status() {
                        StatusCode::OK => {
                            return Ok(result)
                        }
                        _ => {}
                    }
                }
                Err(_) => {}
            }

            log!("Retrying after: {} seconds", delay);
            task::sleep(Duration::from_secs(delay)).await;
            delay += delay;
            retries -= 1;
        }
        
        // If we make it this far we are returning an error or a non-200 response 
        Self::web_get_final(uri).await
    }    

    pub async fn web_post_final(uri: &String, json: &Value) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let req = client.post(uri)
            .json(json)
            .send();
        
        match req.await {
            Ok(response) => {
                log!("[reqwest] POST {:#?}...Ok", uri);
                return Ok(response)
            }
            Err(error) => {
                log!("[reqwest] POST {:#?}...Err", uri);
                return Err(error)
            }
        }
    }

    pub async fn web_post(uri: &String, json: &Value, mut retries: u8) -> Result<Response, Error> {
        let mut delay = 4;
        
        while retries > 0 {
            let result = Self::web_post_final(uri, json).await;

            match result {
                Ok(result) => {
                    match result.status() {
                        StatusCode::OK => {
                            return Ok(result)
                        }
                        _ => {}
                    }
                }
                Err(_) => {}
            }

            log!("Retrying after: {} seconds", delay);
            task::sleep(Duration::from_secs(delay)).await;
            delay += delay;
            retries -= 1;
        }
        
        // If we make it this far we are returning an error or a non-200 response 
        Self::web_post_final(uri, json).await
    }
}  
