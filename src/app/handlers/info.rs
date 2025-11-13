use axum::{Json, response::IntoResponse};

use crate::api_models::{ApiResult, ServerInfo};

pub async fn handle_info() -> ServerInfo {
    ServerInfo::new()
}
