use crate::models::messages::Join;
use actix::{Actor, Context, Handler};
use std::thread::JoinHandle;
use std::time::Instant;
use spin_sleep::LoopHelper;
use futures::sync::oneshot;

#[derive(Debug)]
pub struct GameActor {
	cancel_chan: Option<oneshot::Sender<()>>,
}

impl GameActor {
	pub fn new() -> GameActor {

		GameActor {
			cancel_chan: None,
		}
	}
}

fn game_loop(mut cancel_chan: oneshot::Receiver<()>) {
	let mut counter = 0;
	let mut loop_helper = LoopHelper::builder()
		.build_with_target_rate(30);

	loop {
		loop_helper.loop_start();

		match cancel_chan.try_recv() {
			Ok(Some(_)) | Err(_) => {
				break;
			}
			_ => {}
		}

		if counter > 3000 {
			break;
		}

		// Send out update packets

		loop_helper.loop_sleep();
	}

	println!("game over!");
}

impl Actor for GameActor {
	type Context = Context<GameActor>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("Game Actor started!");
		let (tx, rx) = oneshot::channel();

		std::thread::spawn(move || {
			game_loop(rx);
		});

		self.cancel_chan = Some(tx);
	}
}

impl Handler<Join> for GameActor {
	type Result = ();

	fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) {
		println!("{} wants to connect", msg.name);
	}
}
