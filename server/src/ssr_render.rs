use crate::authentication::{cookie::SessionCookie, SessionManager};
use actix_web::{
    web::{Bytes, Data},
    HttpRequest, HttpResponse, Responder,
};
use futures::stream::{self, StreamExt};
use methane_emissions_management_client::{ServerApp, ServerAppProps};
use std::{convert::Infallible, path::PathBuf};

pub async fn ssr_render(
    req: HttpRequest,
    static_dir: Data<PathBuf>,
    session_cookie_option: Option<SessionCookie>,
    session_manager: Data<SessionManager>,
) -> impl Responder {
    let _redirect_url = if let Some(session_cookie) = session_cookie_option {
        if let Ok(_) = session_manager.user_id(&session_cookie).await {
            req.uri().to_string()
        } else {
            "/register".to_string()
        }
    } else {
        "/register".to_string()
    };

    // let red = session_cookie_option.map_or_else(
    //     || "register".to_string(),
    //     async |session_cookie| {
    //         session_manager
    //             .user_id(&session_cookie)
    //             .await
    //             .map_or_else(|| "/register".to_string(), |a| req.uri().to_string())
    //     },
    // );

    // let condition = {
    //     let url = req.uri().to_string();
    //     println!("url: {}", url);
    //     if url == "/register" {
    //         false
    //     } else {
    //         if let Some(cookie) = auth_cookie {
    //             let session_manager = SessionManager::new(&redis_store);
    //             let user_id_result = session_manager.user_id(&cookie).await;
    //             if let Ok(user_id) = user_id_result {
    //                 println!("user id: {}", user_id);
    //                 false
    //             } else {
    //                 true
    //             }
    //         } else {
    //             true
    //         }
    //     }
    // };

    // if condition {
    //     return HttpResponse::Found()
    //         .append_header(("Location", "/register"))
    //         .finish();
    // }

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
