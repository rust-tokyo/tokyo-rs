use common::models::{BulletState, GameState, PlayerState, GameCommand};
use std::collections::HashMap;

pub struct Game {
    pub state: GameState,
    bullet_id_counter: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState {
                players: vec![],
                bullets: vec![],
                scoreboard: HashMap::new(),
            },
            bullet_id_counter: 0,
        }
    }

    pub fn add_player(&mut self, player_id: u32) {
        self.state.players.push(PlayerState {
            id: player_id,
            angle: 0.0,
            x: 0.0,
            y: 0.0,
        });
    }

    pub fn player_left(&mut self, player_id: u32) {
        info!("Player {} left!", player_id);
    }

    pub fn handle_cmd(&mut self, player_id: u32, cmd: GameCommand) {
        // info!("Player {} sent command {:#?}", player_id, cmd);

        if let Some(player) = self.state.players.iter_mut().find(|p| p.id == player_id) {
            match cmd {
                GameCommand::Rotate(angle) => {
                    player.angle = angle;
                }
                GameCommand::Forward(throttle) => {
                    // Move the triangles
                    let (vel_x, vel_y) = angle_to_vector(player.angle);

                    player.x += vel_x * throttle;
                    player.y += vel_y * throttle;
                }
                Fire => {
                    let bullet_id = self.bullet_id_counter;
                    self.bullet_id_counter += 1;

                    let distance_from_player: f32 = 5.0;
                    let (bullet_x, bullet_y) = angle_to_vector(player.angle);

                    self.state.bullets.push(BulletState {
                        id: bullet_id,
                        player_id: player.id,
                        angle: player.angle,
                        x: player.x + ((bullet_x * distance_from_player)), // TODO(bschwind) - This is broken math
                        y: player.y + ((bullet_y * distance_from_player)),
                    });
                }
            }
        }
    }

    pub fn init(&mut self) {

    }

    pub fn tick(&mut self, _dt: f32) {
        let bullet_speed = 10.0;

        for bullet in &mut self.state.bullets {
            let (vel_x, vel_y) = angle_to_vector(bullet.angle);

            bullet.x += vel_x * bullet_speed;
            bullet.y += vel_y * bullet_speed;
        }
    }
}

fn angle_to_vector(angle: f32) -> (f32, f32) {
    (angle.cos(), angle.sin())
}
