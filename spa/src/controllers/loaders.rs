use crate::models::{
    repositories::*,
    dispatchers::*,
};

use common::*;
use leptos::log;
use openidconnect::AccessToken;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMessagesRes {
    pub result: Vec<Message>,
    pub error_status: Option<String>,
    pub error_message: Option<String>
}

pub struct Loaders {}

impl Loaders {
    pub async fn web_load_queue_message(url: String, count: u8, dequeue: bool, access_token: Option<AccessToken>) -> GetMessagesRes {
        let req_uri: String = format!("{}/api/nxfutil/status", url);
        let req = StatusReq {
            summary: false,
            message_count: count,
            dequeue: dequeue
        };
        let res = WebHelpers::web_post(&req_uri, &serde_json::to_value(req).unwrap(), access_token).await;

        match res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        match res.json().await {
                            Ok(json) => {
                                return GetMessagesRes {
                                    result: json,
                                    error_status: None,
                                    error_message: None,
                                }
                            }
                            Err(error) => {
                                log!("Returning an empty {} because there is no JSON:\n{:#?}", "Vec<Message>", error);
                                return GetMessagesRes {
                                    result: vec![],
                                    error_status: Some("BAD_JSON".to_string()),
                                    error_message: Some("Unable to parse server response to JSON.".to_string()),
                                }
                            }
                        }
                    }
                    _ => {
                        log!("Returning an empty {} because of {:#?} status code.", "Vec<Message>", res.status());
                        let error_message = match res.status().as_u16() {
                            401 => "Unauthorized. Try logging out and back in again.",
                            403 => "Forbidden. If you have recently been granted access try logging out and back in again after a few minutes.",
                            _ => "Request failed. Try sending the request again in a few seconds."
                        };
                        return GetMessagesRes {
                            result: vec![],
                            error_status: Some(res.status().as_u16().to_string()),
                            error_message: Some(error_message.to_string()),
                        }
                    }
                }
            }
            Err(error) => {
                log!("Returning an empty {} because of error:\n{:#?}", "Vec<Message>", error);
                return GetMessagesRes {
                    result: vec![],
                    error_status: Some("ERROR".to_string()),
                    error_message: Some("Request failed. Try sending the request again in a few seconds.".to_string()),
                }
            }
        }
    }

    pub async fn web_load_dispatcher_messages(dispatcher: NextflowDispatcher, count: u8, access_token: Option<AccessToken>) -> GetMessagesRes {
        return Self::web_load_queue_message(dispatcher.api_url, count, false, access_token).await
    }

    pub async fn web_load_github_nextflow_workflow(project: NextflowProject, access_token: Option<AccessToken>) -> Vec<NextflowWorkflow> {
        let res = WebHelpers::web_get(&project.url, access_token).await;

        let files: Vec<GitHubFile> = match res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        match res.json().await {
                            Ok(json) => {
                                json
                            }
                            Err(error) => {
                                log!("Returning an empty {} because there is no JSON:\n{:#?}", "Vec<NextflowWorkflow>", error);
                                vec![]
                            }
                        }
                    }
                    _ => {
                        log!("Returning an empty {} because of {:#?} status code.", "Vec<NextflowWorkflow>", res.status());
                        vec![]
                    }
                }
            }
            Err(error) => {
                log!("Returning an empty {} because of error:\n{:#?}", "Vec<NextflowWorkflow>", error);
                vec![]
            }
        };

        let mut nf_files: Vec<(String, String)> = vec![];
        let mut json_files: Vec<(String, String)> = vec![];
        for file in files {
            if file.r#type == "file" {
                if file.download_url.ends_with(".nf") {
                    nf_files.push((file.name.clone(), file.download_url.clone()));
                }
                if file.download_url.ends_with(".json") {
                    json_files.push((file.name.clone(), file.download_url.clone()));
                }
            }
        }

        let mut nextflow_workflows: Vec<NextflowWorkflow> = vec![];
        for nf_file in &nf_files {
            for json_file in &json_files {
                nextflow_workflows.push(
                    NextflowWorkflow {
                        project: project.clone(),
                        pipeline: NextflowFile {
                            name: nf_file.0.clone(),
                            url: nf_file.1.clone(),
                        },
                        parameters: NextflowFile {
                            name: json_file.0.clone(),
                            url:  json_file.1.clone(),
                        }
                    }
                );
            }
        }

        return nextflow_workflows
    }

    pub async fn web_load_github_nextflow_projects(org: String, repo: String, access_token: Option<AccessToken>) -> Vec<NextflowProject> {
        let uri = format!("https://api.github.com/repos/{}/{}/contents/nextflow/pipelines", org, repo);
        let res = WebHelpers::web_get(&uri, access_token).await;

        let dirs: Vec<GitHubDir> = match res {
            Ok(res) => {
                match res.status() {
                    StatusCode::OK => {
                        match res.json().await {
                            Ok(json) => {
                                json
                            }
                            Err(error) => {
                                log!("Returning an empty {} because there is no JSON:\n{:#?}", "Vec<NextflowProject>", error);
                                vec![]
                            }
                        }
                    }
                    _ => {
                        log!("Returning an empty {} because of {:#?} status code.", "Vec<NextflowProject>", res.status());
                        vec![]
                    }
                }
            }
            Err(error) => {
                log!("Returning an empty {} because of error:\n{:#?}", "Vec<NextflowProject>", error);
                vec![]
            }
        };

        let mut nextflow_projects: Vec<NextflowProject> = vec![];
        for dir in dirs {
            if dir.r#type == "dir" {
                nextflow_projects.push(
                    NextflowProject {
                        org: org.clone(),
                        repo: repo.clone(),
                        name: dir.name,
                        url: dir.url,
                        html_url: dir.html_url,
                    }
                );
            }
        }
        return nextflow_projects
    }

    pub async fn web_load_github_nextflow_workflows(repo: NextflowRepo, access_token: Option<AccessToken>) -> Vec<NextflowWorkflow> {
        let mut projects: Vec<NextflowProject> = vec![];
        projects.append(&mut Self::web_load_github_nextflow_projects(
            repo.org,
            repo.name,
            access_token.clone()
        ).await);

        let mut workflows: Vec<NextflowWorkflow> = vec![];
        for project in projects {
            workflows.append(&mut Self::web_load_github_nextflow_workflow(
                project,
                access_token.clone()
            ).await);
        }

        return workflows
    }
}