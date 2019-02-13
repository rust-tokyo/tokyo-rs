use actix::Message;

#[derive(Debug, Message)]
pub struct Join {
	pub name: String
}
