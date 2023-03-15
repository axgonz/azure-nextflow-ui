pub use serde::{
    Deserialize, 
    Serialize
};

pub use uuid::Uuid;
use web_sys::window;

/// Minified struct for rendering dispatcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextflowDispatcher {
    pub id: Uuid,
    pub api_url: String,
    pub config_url: String,
}

impl NextflowDispatcher {
    pub fn new(id: Uuid, api_url: String, config_url: String) -> Self {
        Self {
            id,
            api_url,
            config_url,
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
                    "https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config".to_string()
                )
            ]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn api_url(&mut self) -> String {
        if !self.items.is_empty() {
            self.items[0].api_url.clone()
        }
        else {
            "".to_string()
        }
    }    

    pub fn config_url(&mut self) -> String {
        if !self.items.is_empty() {
            self.items[0].config_url.clone()
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
