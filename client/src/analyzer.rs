use crate::{
    analyzer::{bullet::Bullet, player::Player},
    geom::*,
};
use common::models::GameState;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub mod bullet;
pub mod player;

// Collision detection etc is done at this compute interval.
pub const ANALYSIS_INTERVAL: Duration = Duration::from_millis(33);

pub struct Analyzer {
    own_player_id: u32,
    players: HashMap<u32, Player>,
    bullets: Vec<Bullet>,
    last_update: Instant,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            own_player_id: 0,
            players: HashMap::new(),
            bullets: Vec::new(),
            last_update: Instant::now(),
        }
    }

    pub fn push_state(&mut self, state: &GameState, time: Instant) {
        let mut players = HashMap::new();
        for player_state in state.players.iter() {
            let player = if let Some(mut prev_player) = self.players.remove(&player_state.id) {
                prev_player.push_state(&player_state, &state.scoreboard, time);
                prev_player
            } else {
                Player::with_state(&player_state, &state.scoreboard, time)
            };
            players.insert(player.id, player);
        }
        self.players = players;

        self.bullets = state
            .bullets
            .iter()
            .map(|state| Bullet::new(&state))
            .collect();

        self.last_update = time;
    }

    pub fn set_own_player_id(&mut self, id: u32) {
        self.own_player_id = id;
    }

    pub fn angle_to(&self, target: &Positioned) -> Radian {
        self.own_player().position.angle_to(&target.position())
    }

    pub fn player<'a>(&'a self, id: u32) -> &'a Player {
        self.players.get(&id).unwrap()
    }

    pub fn own_player<'a>(&'a self) -> &'a Player {
        self.player(self.own_player_id)
    }

    pub fn player_closest(&self) -> &Player {
        let my_position = self.own_player().position;
        self.players
            .values()
            .max_by_key(|player| (my_position.distance(&player.position) * 1e3) as u64)
            .unwrap()
    }

    pub fn player_least_moving(&self) -> &Player {
        self.players
            .values()
            .min_by_key(|player| (player.trajectory.ave_abs_velocity().length() * 1e3) as u64)
            .unwrap()
    }

    pub fn player_highest_score(&self) -> &Player {
        self.players
            .values()
            .max_by_key(|player| player.score())
            .unwrap()
    }

    pub fn player_highest_score_after(&self, after: Duration) -> &Player {
        self.players
            .values()
            .max_by_key(|player| player.score_history.project(after))
            .unwrap()
    }

    pub fn players_within(&self, radius: f32) -> Vec<&Player> {
        let my_position = self.own_player().position;
        self.players
            .values()
            .filter(|player| my_position.distance(&player.position) <= radius)
            .collect::<Vec<_>>()
    }

    pub fn bullets_colliding(&self, during: Duration) -> Vec<&Bullet> {
        self.bullets
            .iter()
            .filter(|bullet| {
                self.own_player()
                    .is_colliding_during(bullet, during.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn bullets_within(&self, radius: f32) -> Vec<&Bullet> {
        let my_position = self.own_player().position;
        self.bullets
            .iter()
            .filter(|bullet| my_position.distance(&bullet.position) <= radius)
            .collect::<Vec<_>>()
    }
}

pub trait Positioned {
    fn position(&self) -> Point;
}
