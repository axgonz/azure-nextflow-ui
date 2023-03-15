use crate::models::{
    repositories::*,
    dispatchers::*,
};

use common::*;

pub struct Loaders {}

impl Loaders {
    pub async fn web_load_queue_message(url: String) -> Vec<Message> { 
        let req_uri: String = format!("{}/api/nxfutil/status", url);
        let req_body = r#"{
                "summary": false,
                "message_count": 32,
                "dequeue": false
            }"#;

        let req_json: Value = serde_json::from_str(req_body).unwrap();
        let res = WebHelpers::web_post(&req_uri, &req_json).await.unwrap();
        let messages: Vec<Message> = res.json().await.unwrap();

        return messages
    }

    pub async fn web_load_dispatcher_messages(dispatcher: NextflowDispatcher) -> Vec<Message> {
        return Self::web_load_queue_message(dispatcher.api_url).await
    }

    pub async fn web_load_github_nextflow_workflow(project: NextflowProject) -> Vec<NextflowWorkflow> {
        let res = WebHelpers::web_get(&project.url).await.unwrap();
        let files: Vec<GitHubFile> = res.json().await.unwrap();

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

    pub async fn web_load_github_nextflow_projects(org: String, repo: String) -> Vec<NextflowProject> {
        let uri = format!("https://api.github.com/repos/{}/{}/contents/nextflow/pipelines", org, repo);
        let res = WebHelpers::web_get(&uri).await.unwrap();
        let dirs: Vec<GitHubDir> = res.json().await.unwrap();
        
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

    pub async fn web_load_github_nextflow_workflows(repo: NextflowRepo) -> Vec<NextflowWorkflow> {
        let mut projects: Vec<NextflowProject> = vec![];
        projects.append(&mut Self::web_load_github_nextflow_projects(
            repo.org, 
            repo.name
        ).await);
    
        let mut workflows: Vec<NextflowWorkflow> = vec![];
        for project in projects {
            workflows.append(&mut Self::web_load_github_nextflow_workflow(
                project
            ).await);
        }
    
        return workflows
    }  
}