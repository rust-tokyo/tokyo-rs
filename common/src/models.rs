#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "e", content = "data")]
pub enum GameCommand {
	#[serde(rename = "join")]
	Join {
		name: String
	},

	#[serde(rename = "disconnect")]
	Disconnect {
		reason: String
	},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
	pub counter: u64,
}
