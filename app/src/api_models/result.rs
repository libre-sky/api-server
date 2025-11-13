use axum::{
    body::Body,
    http::{Response, StatusCode, response::Builder},
};
use serde::Serialize;
use serde_json::to_string;

use crate::api_models::ContentType;

/// a wrapper for service results
pub trait ApiResult {
    fn status_code(self) -> StatusCode;
    fn content(self) -> impl Serialize;
}

pub fn to_response<T>(res: T, ct: ContentType) -> Response<Body>
where
    T: ApiResult + Clone,
{
    content_type(Response::builder(), ct)
        .status(res.clone().status_code())
        .body(serialize(res.content(), ct))
        .unwrap()
}

fn content_type(builder: Builder, ct: ContentType) -> Builder {
    builder.header("content-type", ct.to_string())
}

fn serialize(content: impl Serialize, content_type: ContentType) -> Body {
    match content_type {
        ContentType::Json => Body::new(to_string(&content).unwrap()),
    }
}
