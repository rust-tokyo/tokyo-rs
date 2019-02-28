use crate::actors::ClientWsActor;
use common::models::*;
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use spin_sleep::LoopHelper;
use futures::sync::oneshot;
use std::collections::{HashMap, HashSet};

pub struct GameActor {
	connections: HashSet<Addr<ClientWsActor>>,
	cancel_chan: Option<oneshot::Sender<()>>,
}

impl GameActor {
	pub fn new() -> GameActor {

		GameActor {
			connections: HashSet::new(),
			cancel_chan: None,
		}
	}
}

fn game_loop(game_actor: Addr<GameActor>, mut cancel_chan: oneshot::Receiver<()>) {
	let mut loop_helper = LoopHelper::builder()
		.build_with_target_rate(30);

	let state = GameState {
		players: vec![],
		bullets: vec![],
		scoreboard: HashMap::new(),
	};

	loop {
		loop_helper.loop_start();

		match cancel_chan.try_recv() {
			Ok(Some(_)) | Err(_) => {
				break;
			}
			_ => {}
		}

		// Send out update packets

		game_actor.do_send(state.clone());
		loop_helper.loop_sleep();
	}

	println!("game over!");
}

impl Actor for GameActor {
	type Context = Context<GameActor>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("Game Actor started!");
		let (tx, rx) = oneshot::channel();
		let addr = ctx.address();

		std::thread::spawn(move || {
			game_loop(addr, rx);
		});

		self.cancel_chan = Some(tx);
	}
}

#[derive(Message)]
pub enum SocketEvent {
	Join(Addr<ClientWsActor>),
	Leave(Addr<ClientWsActor>),
}

impl Handler<SocketEvent> for GameActor {
	type Result = ();

	fn handle(&mut self, msg: SocketEvent, _ctx: &mut Self::Context) {
		match msg {
			SocketEvent::Join(addr) => {
				println!("person joined");
				self.connections.insert(addr);
			}
			_ => {}
		}
	}
}

impl Handler<GameState> for GameActor {
	type Result = ();

	fn handle(&mut self, msg: GameState, _ctx: &mut Self::Context) {
		for addr in self.connections.iter() {
			addr.do_send(msg.clone());
		}
	}
}
