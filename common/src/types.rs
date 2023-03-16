pub use serde::{
    Deserialize, 
    Serialize
};

pub use serde_json::{
    Value,
};

pub use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StatusReq {
    pub summary: bool,
    pub message_count: u8,
    pub dequeue: bool
}

/// Part of Message struct: message.metadata.workflow 
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    pub errorMessage: Option<String>
}

/// Part of Message struct: message.metadata 
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub parameters: Value,
    pub workflow: Workflow
}

/// Returned when azure-nextflow 'status' api finds a message
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DispatchReqParam {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DispatchReq {
    pub config_uri: String,
    pub pipeline_uri: String,
    pub parameters_uri: String,
    pub parameters_json: Vec<DispatchReqParam>,
    pub auto_delete: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DispatchRes {
    pub sub_id: String,
    pub rg_name: String,
    pub ci_name: String,
    pub ci_cmd: String,
    pub provisioning_state: String,
}

/// Used in GitHubDir & GitHubFile struct: dir.type & file.type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GitHubFsType {
    Dir,
    File,
}

/// Returned when github 'contents' api finds a directory
/// https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubDir {
    pub r#type: String,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: Option<String>,
} 

/// Returned when github 'contents' api finds a file
/// https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubFile {
    pub r#type: String,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i32,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
}
