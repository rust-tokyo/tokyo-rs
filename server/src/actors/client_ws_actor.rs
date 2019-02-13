use crate::{
	actors::GameActor,
	AppState,
	models::messages::Join,
};
use actix::Addr;
use common::models::{GameCommand, GameState};
use actix::{Actor, ActorContext, StreamHandler};
use actix_web::ws;

pub struct ClientWsActor {
	game_addr: Addr<GameActor>,
}

impl ClientWsActor {
	pub fn new(game_addr: Addr<GameActor>) -> ClientWsActor {
		ClientWsActor {
			game_addr
		}
	}
}

impl Actor for ClientWsActor {
	type Context = ws::WebsocketContext<Self, AppState>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("Log Actor started!");
	}
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ClientWsActor {
	fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
		println!("Message: {:#?}", msg);

		match msg {
			ws::Message::Text(cmd) => {
				let deserialized = serde_json::from_str(&cmd).unwrap();

				match deserialized {
					GameCommand::Join {name} => {
						let connect_msg = Join {
							name
						};

						self.game_addr.do_send(connect_msg);
					}
					GameCommand::Disconnect {reason} => {
						println!("Client left - {}", reason);
					}
				}

				let game_state_string = serde_json::to_string(&GameState {
					counter: 1
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
