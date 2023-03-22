use common::*;
use openidconnect::AccessToken;

use crate::controllers::loaders::*;
use leptos::log;

#[derive(Clone)]
pub struct DispatchWorkflowRes {
    pub result: Vec<DispatchRes>, 
    pub error_status: Option<String>,   
    pub error_message: Option<String>
}

pub struct Actions {}

impl Actions {
    pub async fn web_action_dispatch_workflow(api_url: String, what_if: bool, req: DispatchReq, access_token: Option<AccessToken>) -> DispatchWorkflowRes {
        let req_uri: String = format!("{}/api/nxfutil/dispatch?whatif={}", api_url, what_if.to_string());
        let req_json: Value = serde_json::to_value(req).unwrap();
        let res = WebHelpers::web_post(&req_uri, &req_json, access_token).await;

        match res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        match res.json().await {
                            Ok(json) => {
                                return DispatchWorkflowRes {
                                    result: vec![json],  
                                    error_status: None,
                                    error_message: None
                                }
                            }
                            Err(error) => {
                                log!("Returning an empty {} because there is no JSON:\n{:#?}", "Vec<DispatchRes>", error);
                                return DispatchWorkflowRes {
                                    result: vec![],    
                                    error_status: Some("BAD_JSON".to_string()),
                                    error_message: Some("Unable to parse server response to JSON.".to_string())
                                }
                            }
                        }
                    }
                    _ => {
                        log!("Returning an empty {} because of {:#?} status code.", "Vec<DispatchRes>", res.status());
                        let error_message = match res.status().as_u16() {
                            401 => "Unauthorized. Try logging out and back in again.",
                            _ => "Request failed. Try sending the request again in a few seconds."
                        };
                        return DispatchWorkflowRes {
                            result: vec![],    
                            error_status: Some(res.status().as_u16().to_string()),
                            error_message: Some(error_message.to_string())
                        }
                    }
                }
            }
            Err(error) => {
                log!("Returning an empty {} because of error:\n{:#?}", "Vec<DispatchRes>", error);
                return DispatchWorkflowRes {
                    result: vec![],
                    error_status: Some("ERROR".to_string()),
                    error_message: Some("Request failed. Try sending the request again in a few seconds.".to_string())
                }
            }
        };
    } 

    pub async fn web_action_dispatcher_messages_dequeue(api_url: String, count: u8, access_token: Option<AccessToken>) -> GetMessagesRes {
        return Loaders::web_load_queue_message(api_url, count, true, access_token).await
    }    
}