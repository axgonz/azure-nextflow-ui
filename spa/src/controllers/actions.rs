use common::*;

use super::loaders::Loaders;

pub struct Actions {}

impl Actions {
    pub async fn web_action_dispatch_workflow(api_url: String, what_if: bool, req: DispatchReq) -> DispatchRes {
        let req_uri: String = format!("{}/api/nxfutil/dispatch?whatif={}", api_url, what_if.to_string());
        let req_json: Value = serde_json::to_value(req).unwrap();
        let res = WebHelpers::web_post(&req_uri, &req_json).await.unwrap();
        let result: DispatchRes = res.json().await.unwrap();

        return result
    } 

    pub async fn web_action_dispatcher_messages_dequeue(api_url: String, count: u8, dequeue: bool) -> Vec<Message> {
        return Loaders::web_load_queue_message(api_url, count, dequeue).await
    }    
}