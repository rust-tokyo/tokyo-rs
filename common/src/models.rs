use actix::Message;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "e", content = "data")]
pub enum GameCommand {
    #[serde(rename = "rotate")]
    Rotate(f32), // In radians, no punish.

    #[serde(rename = "throttle")]
    Throttle(f32), // Between -1.0 and 1.0, otherwise consequences.

    #[serde(rename = "fire")]
    Fire, // Between -1.0 and 1.0, otherwise consequences.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    id: u32,
    angle: f32,
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BulletState {
    id: u32,
    player_id: u32,
    angle: f32,
    x: u32,
    y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<PlayerState>,
    pub bullets: Vec<BulletState>,
    pub scoreboard: HashMap<u32, u32>,
}

impl Message for GameState {
    type Result = ();
}
