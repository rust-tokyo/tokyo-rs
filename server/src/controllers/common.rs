use crate::AppState;
use actix_web::{HttpRequest, HttpResponse};
use std::path::PathBuf;

pub fn index_handler(req: HttpRequest<AppState>) -> actix_web::Result<HttpResponse> {
    let _path: PathBuf = req.match_info().query("tail").unwrap_or_else(|_| "".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../frontend/resources/public/index.html")))
}
