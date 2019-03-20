use rand::{thread_rng, Rng};
use tokyo::{self, models::*, Handler};

struct Player {
    id: Option<u32>,
    angle: f32,
    counter: u32,
}

impl Player {
    fn new() -> Self {
        Self { id: None, angle: 0.0, counter: 0 }
    }
}

impl Handler for Player {
    fn tick(&mut self, state: &ClientState) -> Option<GameCommand> {
        if self.id.is_none() {
            self.id = Some(state.id);
        }

        let angle = self.angle;
        self.angle += 0.01;

        self.counter += 1;

        Some(match self.counter % 3 {
            0 => GameCommand::Rotate(angle),
            1 => GameCommand::Fire,
            _ => GameCommand::Forward(1.0),
        })
    }
}

fn main() {
    println!("starting up...");
    let mut rng = thread_rng();
    tokyo::run(
        &rng.gen::<u64>().to_string(),
        &format!("H4CK TH3 PL4N3T {}", rng.gen::<u8>()),
        Player::new(),
    )
    .unwrap();
}
