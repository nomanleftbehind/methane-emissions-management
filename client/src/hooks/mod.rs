// use crate::utils::console_log;
use crate::utils::error::AppError;
use crate::utils::{common::build_request, console_log::console_log};
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;
use yew::{hook, use_effect_with_deps, use_state};

#[derive(Clone, Debug)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub loading: bool,
}

#[hook]
pub fn use_query<Q>(variables: Q::Variables) -> Result<QueryResponse<Q::ResponseData>, AppError>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
{
    let state = use_state(|| {
        Ok(QueryResponse {
            data: None,
            loading: true,
        })
    });

    {
        let state = state.clone();

        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
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
                                Ok(response) => state.set(Ok(QueryResponse {
                                    data: response.data,
                                    loading: false,
                                })),
                                Err(error) => {
                                    console_log!("response 2 error: {:#?}", error);
                                    state.set(Err(error));
                                }
                            }
                        }
                        Err(error) => {
                            console_log!("response 1 error: {:#?}", error);
                            state.set(Err(error));
                        }
                    }
                });
                || ()
            },
            (),
        );
    }
    (*state).clone()
}

pub async fn use_query_async<Q>(
    variables: Q::Variables,
) -> Result<QueryResponse<Q::ResponseData>, AppError>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
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
                Ok(response) => Ok(QueryResponse {
                    data: response.data,
                    loading: false,
                }),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}
