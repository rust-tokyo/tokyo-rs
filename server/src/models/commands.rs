#[derive(Serialize, Deserialize)]
#[serde(tag = "e", content = "data")]
pub enum GameCommand {
	#[serde(rename = "increment")]
	Increment,

	#[serde(rename = "decrement")]
	Decrement,

	#[serde(rename = "move")]
	Move(u32, u32),
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
	pub counter: u64,
}
