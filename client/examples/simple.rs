/// A simple example client that only works with the bare minimal API. If you are
/// new to Rust, or want to build your own logic from the ground up, this is a
/// good start for you.
use rand::{thread_rng, Rng};
use tokyo::{self, models::*, Handler};

#[derive(Default)]
struct Player {
    id: u32,
    angle: f32,
    counter: u32,
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        self.id = state.id;

        let angle = self.angle;
        self.angle += 0.01;

        self.counter += 1;

        Some(match self.counter % 3 {
            0 => GameCommand::Rotate(angle),
            1 => GameCommand::Fire,
            _ => GameCommand::Throttle(1.0),
        })
    }
}

fn main() {
    let mut rng = thread_rng();

    // TODO: Substitute with your API key and team name.
    let api_key = &rng.gen::<u64>().to_string();
    let team_name = &format!("H4CK TH3 PL4N3T {}", rng.gen::<u8>());

    println!("starting up...");
    tokyo::run(api_key, team_name, Player::default()).unwrap();
}
