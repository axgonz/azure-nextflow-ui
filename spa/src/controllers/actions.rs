use common::*;

use super::loaders::Loaders;
use leptos::log;

pub struct Actions {}

impl Actions {
    pub async fn web_action_dispatch_workflow(api_url: String, what_if: bool, req: DispatchReq, access_token: Option<String>) -> Vec<DispatchRes> {
        let req_uri: String = format!("{}/api/nxfutil/dispatch?whatif={}", api_url, what_if.to_string());
        let req_json: Value = serde_json::to_value(req).unwrap();
        let res = WebHelpers::web_post(&req_uri, &req_json, 4, access_token).await;

        match res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        match res.json().await {
                            Ok(json) => {
                                return vec![json]
                            }
                            Err(error) => {
                                log!("Returning an empty {} because there is no JSON:\n{:#?}", "Vec<DispatchRes>", error);
                                return vec![]
                            }
                        }
                    }
                    _ => {
                        log!("Returning an empty {} because of {:#?} status code.", "Vec<DispatchRes>", res.status());
                        return vec![]
                    }
                }
            }
            Err(error) => {
                log!("Returning an empty {} because of error:\n{:#?}", "Vec<DispatchRes>", error);
                return vec![]
            }
        };
    } 

    pub async fn web_action_dispatcher_messages_dequeue(api_url: String, count: u8, access_token: Option<String>) -> Vec<Message> {
        return Loaders::web_load_queue_message(api_url, count, true, access_token).await
    }    
}