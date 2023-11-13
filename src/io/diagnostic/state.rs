use std::sync::Arc;
use axum::extract::State;

pub type AppStateWrapper = State<Arc<AppState>>;

pub struct AppState {
    
}


impl AppState {
    pub fn new() -> Self {
        AppState {

        }
    }
}
