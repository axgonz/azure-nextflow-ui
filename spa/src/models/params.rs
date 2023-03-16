pub use serde::{
    Deserialize, 
    Serialize
};

pub use uuid::Uuid;
use common::types::*;

/// Minified struct for rendering param
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchParam {
    pub id: Uuid,
    pub name: String,
    pub value: String,
}

impl DispatchParam {
    pub fn new(id: Uuid, name: String, value: String) -> Self {
        Self {
            id,
            name,
            value,
        }
    }
}

impl From<&DispatchParam> for DispatchReqParam {
    fn from(param: &DispatchParam) -> Self {
        Self {
            name: param.name.clone(),
            value: param.value.clone().into()
        }
    }
}

/// Struct that works on param list as a shared signal
#[derive(Debug, Clone)]
pub struct DispatchParams {
    pub items: Vec<DispatchParam>
}

impl DispatchParams {
    pub fn new() -> Self {
        Self { 
            items: vec![] 
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: DispatchParam) {
        self.items.push(item)
    }

    pub fn remove(&mut self, id: Uuid) {
        self.items.retain(|item| item.id != id)
    }
}