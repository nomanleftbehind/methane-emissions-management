use actix_web::web::{self, Data};
use actix_web::{get, Responder};
use actix_web::{HttpRequest, HttpResponse};
use emissions_app_client::{ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use std::convert::Infallible;
use std::fs;
use std::path::PathBuf;

#[get("/{tail:.*}")]
pub async fn render(req: HttpRequest, static_dir: Data<PathBuf>) -> impl Responder {
    let index_html_s =
        fs::read_to_string(static_dir.join("index.html")).expect("failed to read index.html");

    let url = req.uri().to_string();

    let renderer =
        yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps { url: url.into() });

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let body = stream::once(async move { index_html_before })
        .chain(renderer.render_stream())
        .chain(stream::once(async move { index_html_after }))
        .map(|html| Ok(web::Bytes::from(html)) as Result<web::Bytes, Infallible>);

    HttpResponse::Ok()
        .content_type("text/html")
        .streaming(body)
}
