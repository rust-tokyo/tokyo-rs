use crate::{
	actors::ApiActor,
	AppState
};
use actix_web::{HttpRequest, Path};


pub fn stream_logs(
	(req): (HttpRequest<AppState>),
) -> Result<actix_web::HttpResponse, actix_web::Error> {
	actix_web::ws::start(&req, ApiActor::new())
}
