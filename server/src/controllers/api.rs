use crate::{
	actors::ClientWsActor,
	AppState
};
use actix_web::{HttpRequest, State};


pub fn socket_handler(
	(req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<actix_web::HttpResponse, actix_web::Error> {
	actix_web::ws::start(&req, ClientWsActor::new(state.game_addr.clone()))
}

pub fn spectate_handler(
	(req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<actix_web::HttpResponse, actix_web::Error> {
	actix_web::ws::start(&req, ClientWsActor::new(state.game_addr.clone()))
}
