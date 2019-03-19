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

#[derive(Debug)]
pub struct Analyzer {
    own_player_id: u32,
    players: HashMap<u32, Player>,
    bullets: Vec<Bullet>,
    last_update: Instant,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self {
            own_player_id: 0,
            players: HashMap::new(),
            bullets: Vec::new(),
            last_update: Instant::now(),
        }
    }
}

impl Analyzer {
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

        self.bullets = state.bullets.iter().map(|state| Bullet::new(&state)).collect();

        self.last_update = time;
    }

    pub fn set_own_player_id(&mut self, id: u32) {
        self.own_player_id = id;
    }

    pub fn player(&self, id: u32) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn own_player(&self) -> &Player {
        // This unwrap() should succeed as long as you don't modify
        // tokyo::build_game_loop function.
        self.player(self.own_player_id).unwrap()
    }

    // TODO: return iterator.
    pub fn other_players(&self) -> Vec<&Player> {
        self.players.values().filter(|player| player.id != self.own_player_id).collect::<Vec<_>>()
    }

    pub fn player_closest(&self) -> Option<&Player> {
        self.other_players()
            .iter()
            .min_by_key(|player| (self.own_player().distance(**player) * 1e3) as u64)
            .map(|player| *player)
    }

    pub fn player_least_moving(&self) -> Option<&Player> {
        self.other_players()
            .iter()
            .min_by_key(|player| (player.trajectory.ave_abs_velocity().length() * 1e3) as u64)
            .map(|player| *player)
    }

    pub fn player_highest_score(&self) -> Option<&Player> {
        self.other_players().iter().max_by_key(|player| player.score()).map(|player| *player)
    }

    pub fn player_highest_score_after(&self, after: Duration) -> Option<&Player> {
        self.other_players()
            .iter()
            .max_by_key(|player| player.score_history.project(after))
            .map(|player| *player)
    }

    pub fn players_within(&self, radius: f32) -> Vec<&Player> {
        self.other_players()
            .iter()
            .filter(|player| self.own_player().distance(**player) <= radius)
            .map(|player| *player)
            .collect::<Vec<_>>()
    }

    pub fn own_bullets_count(&self) -> usize {
        self.bullets.iter().filter(|bullet| bullet.player_id == self.own_player_id).count()
    }

    pub fn bullets_colliding(&self, during: Duration) -> Vec<&Bullet> {
        self.bullets
            .iter()
            .filter(|bullet| self.own_player().is_colliding_during(bullet, during.clone()))
            .collect::<Vec<_>>()
    }

    pub fn bullets_within(&self, radius: f32) -> Vec<&Bullet> {
        self.bullets
            .iter()
            .filter(|bullet| self.own_player().distance(*bullet) <= radius)
            .collect::<Vec<_>>()
    }
}
