// use crate::models::messages::{};
use crate::AppState;
use common::models::{GameCommand, GameState};
use actix::{Actor, ActorContext, StreamHandler};
use actix_web::ws;

pub struct ApiActor {
	// ssh_actor_addr: Option<Addr<SshActor>>,
}

impl ApiActor {
	pub fn new() -> ApiActor {
		ApiActor {
			// ssh_actor_addr: None
		}
	}
}

impl Actor for ApiActor {
	type Context = ws::WebsocketContext<Self, AppState>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("Log Actor started!");
	}
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ApiActor {
	fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
		println!("Message: {:#?}", msg);

		match msg {
			ws::Message::Text(cmd) => {
				let deserialized = serde_json::from_str(&cmd).unwrap();

				match deserialized {
					GameCommand::Increment => {
						let mut state = ctx.state().game_state.lock().unwrap();
						state.counter += 1;
					}
					GameCommand::Decrement => {
						let mut state = ctx.state().game_state.lock().unwrap();
						state.counter -= 1;
					}
					GameCommand::Move(x, y) => {

					}
				}

				let counter = {
					ctx.state().game_state.lock().unwrap().counter
				};

				let game_state_string = serde_json::to_string(&GameState {
					counter
				}).unwrap();

				ctx.text(game_state_string);
			}
			ws::Message::Close(_) => {
				println!("closing the connection!");
				ctx.stop();
			}
			_ => {}
		}
	}
}
