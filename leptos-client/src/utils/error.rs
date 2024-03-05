use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Error)]
pub enum AppError {
    #[error("{0}")]
    RequestError(String),
}

impl From<gloo_net::Error> for AppError {
    fn from(value: gloo_net::Error) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl From<Vec<graphql_client::Error>> for AppError {
    fn from(value: Vec<graphql_client::Error>) -> Self {
        Self::RequestError(
            value
                .into_iter()
                .map(|e| e.message)
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl From<&leptos_router::ParamsError> for AppError {
    fn from(value: &leptos_router::ParamsError) -> Self {
        Self::RequestError(value.to_string())
    }
}
