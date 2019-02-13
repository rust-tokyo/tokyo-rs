use crate::models::messages::Join;
use actix::{Actor, Context, Handler};
use std::thread::JoinHandle;
use std::time::Instant;
use futures::sync::oneshot;

#[derive(Debug)]
pub struct GameActor {
	thread_handle: JoinHandle<()>,
	cancel_chan: oneshot::Sender<()>,
}

impl GameActor {
	pub fn new() -> GameActor {
		let (tx, rx) = oneshot::channel();

		let thread_handle = std::thread::spawn(move || {
			game_loop(rx);
		});

		GameActor {
			thread_handle,
			cancel_chan: tx,
		}
	}
}

fn game_loop(mut cancel_chan: oneshot::Receiver<()>) {
	let mut counter = 0;
	let time_step = 1.0f64 / 60.0f64;
	let mut time_accum = 0.0f64;
	let mut total_time = 0.0f64;
	let mut last_frame_time = Instant::now();

	loop {
		let now = Instant::now();
		let frame_time = now.duration_since(last_frame_time);

		let dt = if frame_time.as_secs() > 0 || frame_time.subsec_micros() > 250_000 {
			0.25f64
		} else {
			frame_time.subsec_micros() as f64 / 1_000_000f64
		};

		last_frame_time = now;
		time_accum += dt;

		while time_accum >= time_step {
			counter += 1;

			// Run the game simulation here
			// println!("Update!: {:?}, {}", counter, total_time);

			total_time += time_step;
			time_accum -= time_step;
		}

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

		std::thread::sleep(std::time::Duration::from_millis(10));
	}

	println!("game over!");
}

impl Actor for GameActor {
	type Context = Context<GameActor>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		println!("Game Actor started!");
	}
}

impl Handler<Join> for GameActor {
	type Result = ();

	fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) {
		println!("{} wants to connect", msg.name);
	}
}
