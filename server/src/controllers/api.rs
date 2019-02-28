use crate::{
	actors::ClientWsActor,
	AppState
};
use actix_web::{HttpRequest, Query, State};

#[derive(Debug, Deserialize)]
pub struct QueryString {
	api_key: String,
}

pub fn socket_handler(
	(req, state, query): (HttpRequest<AppState>, State<AppState>, Query<QueryString>),
) -> Result<actix_web::HttpResponse, actix_web::Error> {
	println!("Query: {:?}", query);

	actix_web::ws::start(&req, ClientWsActor::new(state.game_addr.clone()))
}

pub fn spectate_handler(
	(req, state): (HttpRequest<AppState>, State<AppState>),
) -> Result<actix_web::HttpResponse, actix_web::Error> {
	actix_web::ws::start(&req, ClientWsActor::new(state.game_addr.clone()))
}
