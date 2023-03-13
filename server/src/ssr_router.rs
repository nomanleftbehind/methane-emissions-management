use actix_web::{
    web::{Bytes, Data},
    HttpRequest, HttpResponse, Responder,
};
use emissions_app_client::{ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use std::{convert::Infallible, path::PathBuf};

/// This function reads
pub async fn ssr_render(req: HttpRequest, static_dir: Data<PathBuf>) -> impl Responder {
    let index_html_string = tokio::fs::read_to_string(static_dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let url = req.uri().to_string();

    let renderer =
        yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps { url: url.into() });

    let (index_html_before, index_html_after) = index_html_string.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let body = stream::once(async move { index_html_before })
        .chain(renderer.render_stream())
        .chain(stream::once(async move { index_html_after }))
        .map(|html| Ok(Bytes::from(html)) as Result<Bytes, Infallible>);

    HttpResponse::Ok().content_type("text/html").streaming(body)
}
