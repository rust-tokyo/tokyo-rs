use crate::AppState;
use actix_web::{fs::NamedFile, HttpRequest};
use std::path::PathBuf;

pub fn index_handler(req: HttpRequest<AppState>) -> actix_web::Result<NamedFile> {
    let _path: PathBuf = req.match_info().query("tail").unwrap_or_else(|_| "".into());
    Ok(NamedFile::open("../spectator/index.html")?)
}
