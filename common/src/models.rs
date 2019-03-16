use actix::Message;
use std::collections::HashMap;
use std::time::SystemTime;

pub const BULLET_RADIUS: f32 = 2.0;
pub const PLAYER_RADIUS: f32 = 10.0;

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

pub trait Triangle {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn angle(&self) -> f32;
    fn radius(&self) -> f32;

    fn is_colliding(&self, other: &Triangle) -> bool {
        let d_x = other.x() - self.x();
        let d_y = other.y() - self.y();
        let d_r = other.radius() + self.radius();
        let squared_dist = d_x * d_x + d_y * d_y;
        let squared_radii = d_r * d_r;

        squared_dist < squared_radii
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: u32,
    pub angle: f32,
    pub x: f32,
    pub y: f32,
}

impl Triangle for PlayerState {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn angle(&self) -> f32 { self.angle }
    fn radius(&self) -> f32 { PLAYER_RADIUS }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct BulletState {
    pub id: u32,
    pub player_id: u32,
    pub angle: f32,
    pub x: f32,
    pub y: f32,
}

impl Triangle for BulletState {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn angle(&self) -> f32 { self.angle }
    fn radius(&self) -> f32 { BULLET_RADIUS }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeadPlayer {
    pub respawn: SystemTime,
    pub player: PlayerState,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Message)]
pub struct GameState {
    pub players: Vec<PlayerState>,
    pub dead: Vec<DeadPlayer>,
    pub bullets: Vec<BulletState>,
    pub scoreboard: HashMap<u32, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)]
pub struct ClientState {
    pub id: u32,
    pub game_state: GameState,
}
