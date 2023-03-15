use crate::utils::{common::build_request, error::AppError};
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;
use yew::{hook, use_effect_with_deps, use_state};

#[derive(Clone, Debug, PartialEq)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub error: Option<AppError>,
    pub loading: bool,
}

#[hook]
pub fn use_query<Q>(variables: Q::Variables) -> QueryResponse<Q::ResponseData>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
        loading: true,
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
                                Ok(response) => {
                                    let error = response.errors.map(|v| v.into());
                                    state.set(QueryResponse {
                                        data: response.data,
                                        error,
                                        loading: false,
                                    })
                                }
                                Err(error) => {
                                    state.set(QueryResponse {
                                        data: None,
                                        error: Some(error),
                                        loading: false,
                                    });
                                }
                            }
                        }
                        Err(error) => {
                            state.set(QueryResponse {
                                data: None,
                                error: Some(error),
                                loading: false,
                            });
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
                    error: response.errors.map(|v| v.into()),
                    loading: false,
                }),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

#[hook]
pub fn use_query_with_deps<Q, D>(variables: Q::Variables, deps: D) -> QueryResponse<Q::ResponseData>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
    D: Clone + PartialEq + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
        loading: true,
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
                                Ok(response) => {
                                    let error = response.errors.map(|v| v.into());
                                    state.set(QueryResponse {
                                        data: response.data,
                                        error,
                                        loading: false,
                                    })
                                }
                                Err(error) => {
                                    state.set(QueryResponse {
                                        data: None,
                                        error: Some(error),
                                        loading: false,
                                    });
                                }
                            }
                        }
                        Err(error) => {
                            state.set(QueryResponse {
                                data: None,
                                error: Some(error),
                                loading: false,
                            });
                        }
                    }
                });
                || ()
            },
            deps,
        );
    }
    (*state).clone()
}
