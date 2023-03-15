pub use serde::{
    Deserialize, 
    Serialize
};

pub use uuid::Uuid;
use web_sys::window;

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

    #[allow(dead_code)]
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