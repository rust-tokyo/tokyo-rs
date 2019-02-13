#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

mod actors;
mod controllers;
// mod errors;
// mod integrations;
// mod middleware;
mod models;
// mod schema;

// use crate::{actors::DbActor, middleware::AuthMiddleware};
use std::sync::{Arc, Mutex};
use actix::{Addr, SyncArbiter, System};
use actix_web::{
	http::Method,
	middleware::{
		session::{self, CookieSessionBackend},
		Logger,
	},
	server, App,
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use listenfd::ListenFd;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
	secret: String,
	server_port: Option<u16>,
}

#[derive(Default, Debug)]
pub struct GameState {
	counter: u64,
}

#[derive(Debug)]
pub struct AppState {
	game_state: Arc<Mutex<GameState>>,
}

lazy_static! {
	static ref APP_CONFIG: AppConfig = {
		dotenv().expect("Could not load the .env config file");
		envy::from_env::<AppConfig>().expect("Could not deserialize the .env config file")
	};
}

fn main() -> Result<(), String> {
	lazy_static::initialize(&APP_CONFIG);
	env_logger::init();

	let server_port = APP_CONFIG.server_port.unwrap_or(3000);

	let actor_system = System::new("meetup-server");

	let mut server = server::new(move || {
		let app_state = AppState {
			game_state: Arc::new(Mutex::new(GameState::default()))
		};

		App::with_state(app_state)
			.middleware(Logger::default())
			.resource("/socket", |r| {
				// r.middleware(AuthMiddleware {});
				r.method(Method::GET)
					.with(controllers::api::stream_logs);
			})
			.handler(
				"/static",
				actix_web::fs::StaticFiles::new("frontend/resources/public").unwrap(),
			)
			.resource("/", |r| {
				r.name("home");
				r.method(Method::GET)
					.with(controllers::common::index_handler)
			})
			.resource("/{tail:.*}j", |r| {
				r.method(Method::GET)
					.with(controllers::common::index_handler)
			})
	});

    // Bind to the development file descriptor if available
    // Run with: systemfd --no-pid -s http::3000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();
    server = if let Some(fd) = listenfd.take_tcp_listener(0).unwrap() {
    	server.listen(fd)
    } else {
    	server.bind(format!("0.0.0.0:{}", server_port)).unwrap()
    };

    server.start();

    let _ = actor_system.run();

    Ok(())
}
