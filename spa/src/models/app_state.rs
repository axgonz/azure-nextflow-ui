#[derive(Default, Clone, Debug)]
pub struct AppState {
    pub count: u32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            count: 6,
        }
    }
}