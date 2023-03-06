use crate::utils::console_log::console_log;
use crate::utils::error::AppError;
use gloo_net::{
    http::{Request, RequestCredentials, Response},
    Error,
};
use serde_json::Value;

pub async fn build_request(request_json: &Value) -> Result<Response, AppError> {
    let url = "http://localhost:8081/graphql";

    // Has to be post method because get requests cannot have body.
    let response = Request::post(url)
        .credentials(RequestCredentials::Include)
        .json(request_json)?
        .send()
        .await
        .map_err(Error::into);

    console_log!("res: {:#?}", &response);

    response
}
