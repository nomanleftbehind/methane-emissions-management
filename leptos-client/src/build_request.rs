use crate::error::AppError;
use gloo_net::{
    http::{Request, Response},
    Error,
};
use serde_json::Value;
use web_sys::RequestCredentials;

pub async fn build_request(request_body: &Value) -> Result<Response, AppError> {
    let url = "http://localhost:8081/graphql";

    // Has to be post method because get requests cannot have body.
    Request::post(url)
        .credentials(RequestCredentials::Include)
        .json(request_body)?
        .send()
        .await
        .map_err(Error::into)
}
