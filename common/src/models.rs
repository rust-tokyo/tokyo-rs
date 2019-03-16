use actix::Message;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "e", content = "data")]
pub enum GameCommand {
    #[serde(rename = "rotate")]
    Rotate(f32), // In radians, no punish.

    #[serde(rename = "forward")]
    Forward(f32), // Between -1.0 and 1.0, otherwise consequences.

    #[serde(rename = "fire")]
    Fire, // Between -1.0 and 1.0, otherwise consequences.
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[serde(tag = "e", content = "data")]
pub enum ServerToClient {
    #[serde(rename = "id")]
    Id(u32), // Tell the client their player ID

    #[serde(rename = "state")]
    GameState(GameState), // Send the game state to the client
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: u32,
    pub angle: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct BulletState {
    pub id: u32,
    pub player_id: u32,
    pub angle: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Message)]
pub struct GameState {
    pub players: Vec<PlayerState>,
    pub dead: Vec<u32>,
    pub bullets: Vec<BulletState>,
    pub scoreboard: HashMap<u32, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)]
pub struct ClientState {
    pub id: u32,
    pub game_state: GameState,
}
