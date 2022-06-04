use crate::db::DbPool;
use crate::graphql_module::loader::LoaderRegistry;
use actix_web::web::{self, Data};

pub fn build_schema(
    pool: Data<DbPool>,
    loader_registry: Data<LoaderRegistry>,
    service_provider: Data<ServiceProvider>,
    auth_data: Data<AuthData>,
    settings_data: Data<Settings>,
    restart_switch: Data<Sender<bool>>,
    self_request: Option<Data<Box<dyn SelfRequest>>>,
    include_logger: bool,
) -> Schema {
    let mut builder = schema_builder()
        .data(connection_manager)
        .data(loader_registry)
        .data(service_provider)
        .data(auth_data)
        .data(settings_data)
        .data(restart_switch);

    match self_request {
        Some(self_request) => builder = builder.data(self_request),
        None => {}
    }
    if include_logger {
        builder = builder.extension(Logger).extension(ResponseLogger);
    }
    builder.finish()
}

pub fn config(
    connection_manager: Data<DbPool>,
    loader_registry: Data<LoaderRegistry>,
) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    |cfg| {
        let self_requester: Data<Box<dyn SelfRequest>> = Data::new(Box::new(SelfRequestImpl {
            schema: build_schema(
                connection_manager.clone(),
                loader_registry.clone(),
                service_provider.clone(),
                auth_data.clone(),
                settings_data.clone(),
                restart_switch.clone(),
                None,
                false,
            ),
        }));

        let schema = build_schema(
            connection_manager,
            loader_registry,
            service_provider,
            auth_data,
            settings_data,
            restart_switch,
            Some(self_requester),
            true,
        );

        cfg.app_data(Data::new(schema))
            .service(web::resource("/graphql").guard(guard::Post()).to(
                |schema: Data<Schema>, http_req, req: GraphQLRequest| {
                    graphql(schema, http_req, req)
                },
            ))
            .service(web::resource("/graphql").guard(guard::Get()).to(playground));
    }
}
