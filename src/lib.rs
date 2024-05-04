use axum::extract::FromRef;
use cookie::Key;
use sqlx::SqlitePool;

pub mod auth;
pub mod brp;
pub mod errors;
pub mod utils;
pub mod view;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
