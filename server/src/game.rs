use common::models::{BulletState, GameCommand, GameState, PlayerState};
use std::collections::HashMap;

const BULLET_SPEED: f32 = 10.0;
const BULLET_RADIUS: f32 = 2.0;
const PLAYER_RADIUS: f32 = 10.0;

const BOUNDS_LEFT: f32 = 0.0;
const BOUNDS_RIGHT: f32 = 512.0;
const BOUNDS_TOP: f32 = 0.0;
const BOUNDS_BOTTOM: f32 = 512.0;

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
                    // Move the player
                    let (vel_x, vel_y) = angle_to_vector(player.angle);

                    player.x += vel_x * throttle;
                    player.y += vel_y * throttle;

                    // Keep the players in bounds
                    player.x = player.x.max(BOUNDS_LEFT + PLAYER_RADIUS).min(BOUNDS_RIGHT - PLAYER_RADIUS);
                    player.y = player.y.max(BOUNDS_TOP + PLAYER_RADIUS).min(BOUNDS_BOTTOM - PLAYER_RADIUS);
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
        for bullet in &mut self.state.bullets {
            let (vel_x, vel_y) = angle_to_vector(bullet.angle);

            bullet.x += vel_x * BULLET_SPEED;
            bullet.y += vel_y * BULLET_SPEED;
        }

        // Remove out-of-bound bullets
        self.state.bullets.retain(|b| {
            b.x > (BOUNDS_LEFT - BULLET_RADIUS) &&
            b.x < (BOUNDS_RIGHT + BULLET_RADIUS) &&
            b.y > (BOUNDS_TOP - BULLET_RADIUS) &&
            b.y < (BOUNDS_BOTTOM + BULLET_RADIUS)
        });

        // TODO(bschwind) - find and remove bullet/player intersection pairs
        for bullet in &mut self.state.bullets {
            for player in &mut self.state.players {
                if circles_collide((bullet.x, bullet.y), BULLET_RADIUS, (player.x, player.y), PLAYER_RADIUS) && bullet.player_id != player.id {
                    println!("Bullet {} collided with player {} at ({}, {})", bullet.id, player.id, bullet.x, bullet.y);
                }
            }
        }
    }
}

fn circles_collide((x1, y1): (f32, f32), r1: f32, (x2, y2): (f32, f32), r2: f32) -> bool {
    let squared_dist = ((x2-x1) * (x2-x1)) + ((y2-y1) * (y2-y1));
    let squared_radii = (r1 + r2) * (r1 + r2);

    squared_dist < squared_radii
}

#[test]
fn test_circles_collide() {
    let p1 = (0.0, 0.0);
    let r1 = 1.0;

    let p2 = (2.0, 0.0);
    let r2 = 1.0;

    assert!(!circles_collide(p1, r1, p2, r2));

    let p1 = (0.0, 0.0);
    let r1 = 1.1;

    let p2 = (2.0, 0.0);
    let r2 = 1.0;

    assert!(circles_collide(p1, r1, p2, r2));

    let p1 = (0.0, 0.0);
    let r1 = 3.0;

    let p2 = (6.0, 0.0);
    let r2 = 3.0;

    assert!(!circles_collide(p1, r1, p2, r2));

    let p1 = (0.0, 0.0);
    let r1 = 1.0;

    let p2 = (2.0, 2.0);
    let r2 = 1.0;

    assert!(!circles_collide(p1, r1, p2, r2));

    let p1 = (5.0, 4.0);
    let r1 = 7.4;

    let p2 = (-2.0, -0.3);
    let r2 = 1.0;

    assert!(circles_collide(p1, r1, p2, r2));
}

fn angle_to_vector(angle: f32) -> (f32, f32) {
    (angle.cos(), angle.sin())
}
