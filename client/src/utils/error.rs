use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum AppError {
    RequestError(String),
}

impl From<gloo_net::Error> for AppError {
    fn from(value: gloo_net::Error) -> Self {
        Self::RequestError(value.to_string())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AppError::RequestError(s) = self;
        write!(f, "{}", s)
    }
}
