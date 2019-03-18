use crate::{
    geom::*,
    util::{Bullet, Player},
};
use common::models::GameState;
use std::{collections::HashMap, time::Instant};

pub struct Radar {
    own_player_id: u32,
    players: HashMap<u32, Player>,
    bullets: Vec<Bullet>,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            own_player_id: 0,
            players: HashMap::new(),
            bullets: Vec::new(),
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
    }

    pub fn player<'a>(&'a self, id: u32) -> &'a Player {
        self.players.get(&id).unwrap()
    }

    pub fn set_own_player_id(&mut self, id: u32) {
        self.own_player_id = id;
    }

    pub fn own_player<'a>(&'a self) -> &'a Player {
        self.player(self.own_player_id)
    }

    pub fn angle_to(&self, target: u32) -> Radian {
        self.own_player()
            .position
            .angle_to(&self.player(target).position)
    }

    pub fn bullets_to_collide(&self, _until: Instant) -> Vec<Bullet> {
        // TODO
        unimplemented!();
    }
}
