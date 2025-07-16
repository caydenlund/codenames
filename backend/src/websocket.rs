use actix::{Actor, ActorContext, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, Result, web};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "board_update")]
    BoardUpdate { data: serde_json::Value },

    #[serde(rename = "card_revealed")]
    CardRevealed { data: CardRevealData },

    #[serde(rename = "game_reset")]
    GameReset { data: serde_json::Value },
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

pub struct WebsocketConnection {
    pub id: usize,
    pub tx: broadcast::Sender<WsMessage>,
}

impl Actor for WebsocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Websocket connection `{}` started", self.id);

        let mut rx = self.tx.subscribe();
        let addr = ctx.address();

        actix::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                addr.do_send(BroadcastMessage(msg));
            }
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Websocket connection `{}` stopped", self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketConnection {
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

impl Handler<BroadcastMessage> for WebsocketConnection {
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
    pub broadcast_tx: broadcast::Sender<WsMessage>,
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

    pub fn broadcast(&self, message: WsMessage) {
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
    let id = ws_state.get_next_id();
    let ws_conn = WebsocketConnection {
        id,
        tx: ws_state.broadcast_tx.clone(),
    };
    ws::start(ws_conn, &req, stream)
}
