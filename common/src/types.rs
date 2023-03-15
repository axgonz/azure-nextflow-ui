pub use serde::{
    Deserialize, 
    Serialize
};

pub use serde_json::{
    Value,
};

pub use uuid::Uuid;
pub use web_sys::window;
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

/// Part of NextflowWorkflow struct: workflow.project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NextflowProject {
    pub org: String,
    pub repo: String,
    pub name: String,
    pub url: String,
    pub html_url: String,
}

/// Part of NextflowWorkflow struct: workflow.pipeline & workflow.parameters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NextflowFile {
    pub name: String,
    pub url: String,
}

/// Minified struct for rendering workflow
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NextflowWorkflow {
    pub project: NextflowProject,
    pub pipeline: NextflowFile,
    pub parameters: NextflowFile,
}

/// Minified struct for rendering repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextflowRepo {
    pub id: Uuid,
    pub org: String,
    pub name: String,
}

impl NextflowRepo {
    pub fn new(id: Uuid, org: String, name: String) -> Self {
        Self {
            id,
            org,
            name,
        }
    }
}

/// Struct that works on repositories list as a shared signal
#[derive(Debug, Clone)]
pub struct NextflowRepos {
    pub items: Vec<NextflowRepo>
}

impl NextflowRepos {
    pub fn new() -> Self {
        Self::mock_load()
    }

    pub fn load(storage_key: &String) -> Self {
        let items: Vec<NextflowRepo> = if let Ok(Some(storage)) = window().unwrap().local_storage() {
            storage
                .get_item(&storage_key)
                .ok()
                .flatten()
                .and_then(|value| {
                    serde_json::from_str::<Vec<NextflowRepo>>(&value).ok()
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        Self { items }
    }

    pub fn mock_load() -> Self {
        Self {
            items: vec![
                NextflowRepo::new(
                    Uuid::new_v4(),
                    "axgonz".to_string(),
                    "azure-nextflow".to_string(),
                )
            ]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: NextflowRepo) {
        self.items.push(item)
    }

    pub fn remove(&mut self, id: Uuid) {
        self.items.retain(|item| item.id != id)
    }
}

/// Minified struct for rendering dispatcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextflowDispatcher {
    pub id: Uuid,
    pub url: String,
}

impl NextflowDispatcher {
    pub fn new(id: Uuid, url: String) -> Self {
        Self {
            id,
            url,
        }
    }
}

/// Struct that works on dispatchers list as a shared signal
#[derive(Debug, Clone)]
pub struct NextflowDispatchers {
    pub items: Vec<NextflowDispatcher>
}

impl NextflowDispatchers {
    pub fn new() -> Self {
        Self::mock_load()
    }

    pub fn load(storage_key: &String) -> Self {
        let items: Vec<NextflowDispatcher> = if let Ok(Some(storage)) = window().unwrap().local_storage() {
            storage
                .get_item(&storage_key)
                .ok()
                .flatten()
                .and_then(|value| {
                    serde_json::from_str::<Vec<NextflowDispatcher>>(&value).ok()
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        Self { items }
    }

    pub fn mock_load() -> Self {
        Self {
            items: vec![
                NextflowDispatcher::new(
                    Uuid::new_v4(),
                    "http://localhost:7071".to_string(),
                )
            ]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn url(&mut self) -> String {
        if !self.items.is_empty() {
            self.items[0].url.clone()
        }
        else {
            "".to_string()
        }
    }

    pub fn add(&mut self, item: NextflowDispatcher) {
        self.items.push(item)
    }

    pub fn remove(&mut self, id: Uuid) {
        self.items.retain(|item| item.id != id)
    }
}








/// WARNING To be removed
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Repo {
    pub index: usize,
    pub org: String,
    pub name: String,
}