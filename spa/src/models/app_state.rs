use common::Repo;

#[derive(Default, Clone, Debug)]
pub struct AppState {
    pub count: u32,
    pub repos: Vec<Repo>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            count: 6,
            repos: vec![
                Repo {
                    index: 0,
                    org: "axgonz".to_string(),
                    name: "azure-nextflow".to_string(),
                },
                Repo {
                    index: 1,
                    org: "axgonz".to_string(),
                    name: "azure-nextflow-cipa".to_string(),
                },
            ],
        }
    }
}