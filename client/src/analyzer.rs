use crate::{
    analyzer::{bullet::Bullet, player::Player},
    geom::*,
};
use common::models::ClientState;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub mod bullet;
pub mod player;

// Collision detection etc is done at this compute interval.
pub const ANALYSIS_INTERVAL: Duration = Duration::from_millis(10);

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
    pub fn push_state(&mut self, state: &ClientState, time: Instant) {
        self.own_player_id = state.id;

        let mut players = HashMap::new();
        for player_state in state.game_state.players.iter() {
            let player = if let Some(mut prev_player) = self.players.remove(&player_state.id) {
                prev_player.push_state(&player_state, &state.game_state.scoreboard, time);
                prev_player
            } else {
                Player::with_state(&player_state, &state.game_state.scoreboard, time)
            };
            players.insert(player.id, player);
        }
        self.players = players;

        self.bullets = state.game_state.bullets.iter().map(|state| Bullet::new(&state)).collect();

        self.last_update = time;
    }

    pub fn player(&self, id: u32) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn own_player(&self) -> &Player {
        // This unwrap() should succeed as long as you don't modify
        // tokyo::build_game_loop function.
        self.player(self.own_player_id).unwrap()
    }

    // conservative_impl_trait will help get rid of Box.
    // https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
    pub fn other_players<'a>(&'a self) -> Box<Iterator<Item = &'a Player> + 'a> {
        Box::new(self.players.values().filter(move |player| player.id != self.own_player_id))
    }

    pub fn player_closest(&self) -> Option<&Player> {
        self.other_players().min_by_key(|player| (self.own_player().distance(*player) * 1e3) as u64)
    }

    pub fn player_least_moving(&self) -> Option<&Player> {
        self.other_players()
            .min_by_key(|player| (player.trajectory.ave_abs_velocity().length() * 1e3) as u64)
    }

    pub fn player_highest_score(&self) -> Option<&Player> {
        self.other_players().max_by_key(|player| player.score())
    }

    pub fn player_highest_score_after(&self, after: Duration) -> Option<&Player> {
        self.other_players().max_by_key(|player| player.score_history.project(after))
    }

    pub fn players_within<'a>(&'a self, radius: f32) -> Box<Iterator<Item = &'a Player> + 'a> {
        Box::new(
            self.other_players()
                .filter(move |player| self.own_player().distance(*player) <= radius),
        )
    }

    pub fn own_bullets_count(&self) -> usize {
        self.bullets.iter().filter(|bullet| bullet.player_id == self.own_player_id).count()
    }

    pub fn bullets_colliding<'a>(
        &'a self,
        during: Duration,
    ) -> Box<Iterator<Item = &'a Bullet> + 'a> {
        Box::new(
            self.bullets.iter().filter(move |bullet| {
                self.own_player().is_colliding_during(bullet, during.clone())
            }),
        )
    }

    pub fn bullets_within<'a>(&'a self, radius: f32) -> Box<Iterator<Item = &'a Bullet> + 'a> {
        Box::new(
            self.bullets.iter().filter(move |bullet| self.own_player().distance(*bullet) <= radius),
        )
    }
}
