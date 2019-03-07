use actix::Message;
use common::models::GameCommand;

#[derive(Debug, Message)]
pub struct PlayerGameCommand {
    pub api_key: String,
    pub cmd: GameCommand,
}

#[derive(Debug, Message)]
pub struct ClientStop {}
