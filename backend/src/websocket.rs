use actix::{Actor, ActorContext, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, Result, web};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClientType {
    Public,
    Spymaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "card_revealed")]
    CardRevealed { data: CardRevealData },

    #[serde(rename = "new_game")]
    NewGame { data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardRevealData {
    pub row: usize,
    pub col: usize,
    pub new_card_state: serde_json::Value,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub WsMessage);

pub struct WsConnection {
    pub id: usize,
    pub client_type: ClientType,
    pub tx: broadcast::Sender<(WsMessage, Option<ClientType>)>,
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!(
            "Websocket connection `{}` (`{:?}`) started",
            self.id, self.client_type
        );

        let mut rx = self.tx.subscribe();
        let addr = ctx.address();
        let client_type = self.client_type;

        actix::spawn(async move {
            while let Ok((msg, tgt)) = rx.recv().await {
                match tgt {
                    Some(typ) => {
                        if typ == client_type {
                            addr.do_send(BroadcastMessage(msg))
                        }
                    }
                    None => addr.do_send(BroadcastMessage(msg)),
                }
            }
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Websocket connection `{}` stopped", self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Text(text)) => {
                println!("Received text: `{text}`");
            }
            Ok(ws::Message::Binary(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Err(e) => {
                println!("Websocket error: `{e}`");
                ctx.stop();
            }
            _ => {
                println!("Unhandled message type: `{msg:?}`");
            }
        }
    }
}

impl Handler<BroadcastMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        // Serialize and send the message to the client
        match serde_json::to_string(&msg.0) {
            Ok(json) => ctx.text(json),
            Err(e) => println!("Failed to serialize Websocket message: `{e}`"),
        }
    }
}

#[derive(Clone)]
pub struct WsState {
    pub next_conn_id: Arc<Mutex<usize>>,
    pub broadcast_tx: broadcast::Sender<(WsMessage, Option<ClientType>)>,
}

impl WsState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            next_conn_id: Arc::new(Mutex::new(0)),
            broadcast_tx: tx,
        }
    }

    pub fn get_next_id(&self) -> usize {
        let mut id = self.next_conn_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn broadcast(&self, message: (WsMessage, Option<ClientType>)) {
        if let Err(e) = self.broadcast_tx.send(message) {
            println!("Failed to broadcast message: `{e}`");
        }
    }
}

impl Default for WsState {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    ws_state: web::Data<WsState>,
) -> Result<HttpResponse> {
    let client_type = if req.query_string().contains("type=spymaster") {
        ClientType::Spymaster
    } else {
        ClientType::Public
    };
    let id = ws_state.get_next_id();
    let ws_conn = WsConnection {
        id,
        client_type,
        tx: ws_state.broadcast_tx.clone(),
    };
    ws::start(ws_conn, &req, stream)
}
