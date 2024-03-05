use crate::utils::{build_request, error::AppError};
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;

pub async fn load_data<Q>(variables: Q::Variables) -> Result<Q::ResponseData, AppError>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + std::fmt::Debug + 'static,
{
    let request_body = Q::build_query(variables);
    let request_json = &json!(request_body);
    let request = build_request(request_json).await;

    match request {
        Ok(response) => {
            // json method cannot be called in build_request() function because response type has to implement Deserialize trait which compiler cannot infer.
            let json = response
                .json::<Response<Q::ResponseData>>()
                .await
                .map_err(AppError::from);
            match json {
                Ok(response) => response.data.ok_or_else(|| {
                    response
                        .errors
                        .map_or_else(|| "Unknown error.".into(), AppError::from)
                }),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}
