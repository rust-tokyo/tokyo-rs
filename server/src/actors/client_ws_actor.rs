use crate::models::messages::{ClientStop, PlayerGameCommand};
use crate::{actors::GameActor, AppState};
use actix::{Actor, ActorContext, StreamHandler};
use actix::{Addr, AsyncContext, Handler};
use actix_web::ws;
use actix_web::ws::{CloseCode, CloseReason};
use common::models::ServerToClient;
use ratelimit_meter::{DirectRateLimiter, GCRA};

const ACTIONS_PER_SECOND: u32 = 10;

#[derive(Debug)]
pub struct ClientWsActor {
    game_addr: Addr<GameActor>,
    api_key: String,
    rate_limiter: DirectRateLimiter<GCRA>,
}

impl ClientWsActor {
    pub fn new(game_addr: Addr<GameActor>, api_key: String) -> ClientWsActor {
        let rate_limiter = DirectRateLimiter::<GCRA>::per_second(
            std::num::NonZeroU32::new(ACTIONS_PER_SECOND).unwrap(),
        );

        ClientWsActor {
            game_addr,
            api_key,
            rate_limiter,
        }
    }
}

impl Actor for ClientWsActor {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.game_addr
            .do_send(crate::actors::game_actor::SocketEvent::Join(
                self.api_key.clone(),
                ctx.address(),
            ));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.game_addr
            .do_send(crate::actors::game_actor::SocketEvent::Leave(
                self.api_key.clone(),
                ctx.address(),
            ));
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ClientWsActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(cmd) => {
                if self.rate_limiter.check().is_ok() {
                    let cmd_result = serde_json::from_str(&cmd);

                    if let Ok(cmd) = cmd_result {
                        self.game_addr.do_send(PlayerGameCommand {
                            api_key: self.api_key.clone(),
                            cmd,
                        });
                    }
                } else {
                    warn!("API key {} got rate limited", self.api_key);
                }
            }
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl Handler<ServerToClient> for ClientWsActor {
    type Result = ();

    fn handle(&mut self, msg: ServerToClient, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl Handler<ClientStop> for ClientWsActor {
    type Result = ();

    fn handle(&mut self, _: ClientStop, ctx: &mut Self::Context) {
        ctx.close(Some(CloseReason {
            code: CloseCode::Normal,
            description: Some("The server decided it didn't like you anymore. Or maybe you connected another client with the same API key".to_string())
        }));
    }
}
