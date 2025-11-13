use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ContentType {
    Json,
}
impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            Self::Json => "application/json".to_string(),
        }
    }
}
