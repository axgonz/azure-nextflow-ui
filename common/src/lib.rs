pub mod types;
pub use types::*; 

use leptos::log;

use std::time::Duration;
use async_std::task;

pub use reqwest::{
    Response,
    Error,
    StatusCode,
    header,
};

pub struct WebHelpers {}

impl WebHelpers {
    pub async fn web_get_final(
        uri: &String, access_token: Option<String>
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let req = match access_token{
            Some(access_token) => {
                client.get(uri)
                    .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
            }
            None => {
                client.get(uri)
            }
        };
        
        match req.send().await {
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

    pub async fn web_get(
        uri: &String, mut retries: u8, access_token: Option<String>
    ) -> Result<Response, Error> {
        let mut delay = 4;
        
        while retries > 0 {
            let result = Self::web_get_final(uri, access_token.clone()).await;
       
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
        Self::web_get_final(uri, access_token).await
    }    

    pub async fn web_post_final(
        uri: &String, json: &Value, access_token: Option<String>
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let req = match access_token{
            Some(access_token) => {
                client.post(uri)
                    .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
                    .json(json)
            }
            None => {
                client.post(uri)
                    .json(json)
            }
        };

        match req.send().await {
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

    pub async fn web_post(
        uri: &String, json: &Value, mut retries: u8, access_token: Option<String>
    ) -> Result<Response, Error> {
        let mut delay = 4;
        
        while retries > 0 {
            let result = Self::web_post_final(uri, json, access_token.clone()).await;

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
        Self::web_post_final(uri, json, access_token).await
    }
}  
