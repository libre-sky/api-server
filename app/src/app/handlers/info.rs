use crate::api_models::ServerInfo;

pub async fn handle_info() -> ServerInfo {
    ServerInfo::new()
}
