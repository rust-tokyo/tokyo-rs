use common::models::*;

struct Player;
impl tokyo::Handler for Player {
    fn tick(&mut self, state: &GameState) -> GameCommand {
        println!("{:#?}", state);
        GameCommand::Join { name: "beepboop".into() }
    }
}

fn main() {
    println!("starting up...");
    tokyo::run(Player {}).unwrap();
}
