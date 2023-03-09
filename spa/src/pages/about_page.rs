use crate::models::{
    app_state::AppState
};

use leptos::*;
use reqwest::{
    Response,
    Error,
    header::ACCESS_CONTROL_ALLOW_ORIGIN,
    header::ORIGIN,
    header::CONTENT_TYPE
};

use serde::{
    Deserialize, 
    Serialize
};

use serde_json::{
    Value,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub errorMessage: Option<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub parameters: Value,
    pub workflow: Workflow
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String,
    pub metadata: Metadata,
}

// #[component]
// pub fn ShowData(
//     cx: Scope, 
//     value: ReadSignal<i32>,
//     #[prop(optional)]
//     message: Vec<Message>,
// ) -> impl IntoView {
//     view! { cx,
//         <div>
//             message.event
//         </div>
//     }
// }

async fn web_get(uri: &String) -> Response {
    let response = match reqwest::get(uri).await {
        Ok(response) => {
            response
        }
        Err(error) => {
            println!("[reqwest] GET {:#?}...Err", uri);
            panic!("{}", error)
        }
    };
    if response.status() == 200 {
        println!("[reqwest] GET {:#?}...Ok", uri);
        return response
    }
    else {
        println!("[reqwest] GET {:#?}...Err", uri);
        //ToDo send_message to queue before exiting. 
        panic!("{}", response.status())
    }
}

async fn web_post(uri: &String, json: &Value) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    match client.post(uri)
        .header(ORIGIN, "http://127.0.0.1:8080")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "http://127.0.0.1:8080")
        .json(json)
        .send()
        .await {
        Ok(response) => {
            println!("[reqwest] POST {:#?}...Ok", uri);
            return Ok(response)
        }
        Err(error) => {
            println!("[reqwest] POST {:#?}...Err", uri);
            return Err(error)
        }
    };
}   

async fn load_data(value: i32) -> i32 {
    // https://api.github.com/repos/{{owner}}/{{repo}}/contents/nextflow/pipelines 
 
    let req_uri: String = String::from("http://localhost:7071/api/nxfutil/status");
    let req_body = r#"{
            "summary": true,
            "message_count": 32,
            "dequeue": false
        }"#;

    let req_json: Value = serde_json::from_str(req_body).unwrap();
    let res = web_post(&req_uri, &req_json).await.unwrap();

    log!("{:#?}", res);

    // let messages: Vec<Message> = res.json().await.unwrap();

    return value * 100
}

async fn m_load_data(value: i32) -> i32 {
    // fake a one-second delay
    // TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(cx, 0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        cx,
        // the first is the "source signal"
        move || count.get(),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(cx, || (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    // let loading = move || async_data.loading();
    // let is_loading = move || if loading { "Loading..." } else { "Idle." };

    let on_click = move |_| set_count.update(|n| *n += 1);

    view! { cx,
        <button on:click=on_click>"Click me"</button>
        <p>
            <code>"stable"</code>": " {move || stable.read(cx)}
        </p>
        <p>
            <code>"count"</code>": " {move || count.get()}
        </p>
        <p>
            <code>"async_value"</code>": "{async_result}
            <br/>
            // {is_loading}
        </p>
    }
}