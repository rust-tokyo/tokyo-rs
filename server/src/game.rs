use common::models::{BulletState, GameCommand, DeadPlayer, GameState, PlayerState, Triangle};
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

const DEAD_PUNISH: Duration = Duration::from_secs(5);

const BULLET_SPEED: f32 = 10.0;
const BULLET_RADIUS: f32 = 2.0;
const PLAYER_RADIUS: f32 = 10.0;

const BOUNDS: (f32, f32) = (512.0, 512.0);

#[derive(Default)]
pub struct Game {
    pub state: GameState,
    rng: rand::rngs::ThreadRng,
    bullet_id_counter: u32,
}

impl Game {
    pub fn add_player(&mut self, player_id: u32) {
        let mut player = PlayerState::new(player_id);
        player.randomize(&mut self.rng, BOUNDS);
        self.state.players.push(player);
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
                    // Move the player
                    let (vel_x, vel_y) = angle_to_vector(player.angle);

                    player.x += vel_x * throttle;
                    player.y += vel_y * throttle;

                    // Keep the players in bounds
                    player.x = player.x.max(PLAYER_RADIUS).min(BOUNDS.0 - PLAYER_RADIUS);
                    player.y = player.y.max(PLAYER_RADIUS).min(BOUNDS.1 - PLAYER_RADIUS);
                }
                GameCommand::Fire => {
                    let bullet_id = self.bullet_id_counter;
                    self.bullet_id_counter += 1;

                    let distance_from_player: f32 = 5.0;
                    let (bullet_x, bullet_y) = angle_to_vector(player.angle);

                    self.state.bullets.push(BulletState {
                        id: bullet_id,
                        player_id: player.id,
                        angle: player.angle,
                        x: player.x + (bullet_x * distance_from_player),
                        y: player.y + (bullet_y * distance_from_player),
                    });
                }
            }
        }
    }

    pub fn init(&mut self) {}

    pub fn tick(&mut self, _dt: f32) {
        // Revive the dead
        let now = SystemTime::now();
        let revived = self.state.dead
            .drain_filter(|corpse| corpse.respawn <= now)
            .map(|dead| dead.player)
            .map(|player| {
                println!("revived player {}", player.id);
                player
            });

        self.state.players.extend(revived);

        // Advance bullets
        for bullet in &mut self.state.bullets {
            let (vel_x, vel_y) = angle_to_vector(bullet.angle);

            bullet.x += vel_x * BULLET_SPEED;
            bullet.y += vel_y * BULLET_SPEED;
        }

        // Remove out-of-bound bullets
        self.state.bullets.retain(|b| {
            b.x > (BULLET_RADIUS) &&
            b.x < (BOUNDS.0 + BULLET_RADIUS) &&
            b.y > (BULLET_RADIUS) &&
            b.y < (BOUNDS.1 + BULLET_RADIUS)
        });

        // count the dead
        let mut hits = HashSet::new();
        for bullet in &mut self.state.bullets {
            let deceased = self.state.players.drain_filter(|player| {
                if player.is_colliding(bullet) && bullet.player_id != player.id {
                    println!("Player {} killed player {} at ({}, {})", bullet.player_id, player.id, bullet.x, bullet.y);
                    hits.insert(bullet.player_id);
                    true
                } else {
                    false
                }
            });
            for mut player in deceased {
                player.randomize(&mut self.rng, BOUNDS);
                self.state.dead.push(DeadPlayer {
                    respawn: SystemTime::now() + DEAD_PUNISH,
                    player
                });
            }
        }

        // Update the scoreboard
        for player_id in hits {
            *self.state.scoreboard.entry(player_id).or_default() += 1;
        }
    }
}

// TODO(jake): rewrite tests.... maybe

fn angle_to_vector(angle: f32) -> (f32, f32) {
    (angle.cos(), angle.sin())
}
