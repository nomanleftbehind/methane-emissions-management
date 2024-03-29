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

impl From<Vec<graphql_client::Error>> for AppError {
    fn from(value: Vec<graphql_client::Error>) -> Self {
        Self::RequestError(
            value
                .into_iter()
                .map(|e| format!("{:#?}", e.message))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AppError::RequestError(s) = self;
        write!(f, "{}", s)
    }
}
