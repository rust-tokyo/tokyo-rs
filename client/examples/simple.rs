use common::models::*;

struct Player;
impl tokyo::Handler for Player {
    fn tick(&mut self, state: &GameState) -> GameCommand {
        println!("{:#?}", state);
        GameCommand::Rotate(0.0)
    }
}

fn main() {
    println!("starting up...");
    tokyo::run("7DEA6163-7532-4420-9ECC-10773347DCAE", Player {}).unwrap();
}
