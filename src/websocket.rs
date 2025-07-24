use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const HEARTBEAT: Duration = Duration::from_secs(10);
const TIMEOUT: Duration = Duration::from_secs(20);

// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    CardRevealed { data: CardRevealData },
    NewGame { data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardRevealData {
    pub row: usize,
    pub col: usize,
    pub new_card_state: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClientType {
    Public,
    Spymaster,
}

// Connection info stored in WsState
#[derive(Debug, Clone)]
struct ConnectionInfo {
    client_type: ClientType,
    last_pong: Instant,
    addr: actix::Addr<WebSocketSession>,
}

// Shared state for managing connections
#[derive(Debug, Clone)]
pub struct WsState {
    connections: Arc<Mutex<HashMap<u64, ConnectionInfo>>>,
    next_id: Arc<AtomicU64>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn add_connection(
        &self,
        id: u64,
        client_type: ClientType,
        addr: actix::Addr<WebSocketSession>,
    ) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(
            id,
            ConnectionInfo {
                client_type,
                last_pong: Instant::now(),
                addr,
            },
        );
    }

    pub fn remove_connection(&self, id: &u64) {
        let mut connections = self.connections.lock().unwrap();
        connections.remove(id);
    }

    pub fn update_pong_time(&self, id: &u64) {
        let mut connections = self.connections.lock().unwrap();
        if let Some(info) = connections.get_mut(id) {
            info.last_pong = Instant::now();
        }
    }

    pub fn next_connection_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn broadcast(&self, message: (WsMessage, Option<ClientType>)) {
        let (msg, target_type) = message;
        let connections = self.connections.lock().unwrap();

        for info in connections.values() {
            if let Some(target) = target_type {
                if info.client_type != target {
                    continue;
                }
            }

            if let Err(e) = info.addr.try_send(BroadcastMessage(msg.clone())) {
                log::warn!("Failed to send message to client: {:?}", e);
            }
        }
    }

    // Cleanup stale connections and ping active ones
    pub async fn cleanup_and_ping(&self) {
        let mut to_remove = Vec::new();
        let mut to_ping = Vec::new();

        {
            let mut connections = self.connections.lock().unwrap();
            let now = Instant::now();

            // Find connections to remove (no pong for 60 seconds) and ping active ones
            for (id, info) in connections.iter() {
                if now.duration_since(info.last_pong) > Duration::from_secs(60) {
                    to_remove.push(*id);
                } else {
                    to_ping.push(info.addr.clone());
                }
            }

            // Remove stale connections
            for id in &to_remove {
                connections.remove(id);
            }
        }

        if !to_remove.is_empty() {
            log::info!("Cleaned up {} stale WebSocket connections", to_remove.len());
        }

        // Send ping to active connections
        for addr in to_ping {
            if let Err(e) = addr.try_send(SendPing) {
                log::warn!("Failed to ping client: {:?}", e);
            }
        }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
}

// Actor messages
#[derive(Message)]
#[rtype(result = "()")]
struct BroadcastMessage(WsMessage);

#[derive(Message)]
#[rtype(result = "()")]
struct SendPing;

// WebSocket session actor
pub struct WebSocketSession {
    id: u64,
    client_type: ClientType,
    ws_state: web::Data<WsState>,
    heartbeat: Instant,
}

impl WebSocketSession {
    fn new(client_type: ClientType, ws_state: web::Data<WsState>) -> Self {
        let id = ws_state.next_connection_id();
        Self {
            id,
            client_type,
            ws_state,
            heartbeat: Instant::now(),
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > TIMEOUT {
                log::info!(
                    "WebSocket client {} heartbeat failed, disconnecting",
                    act.id
                );
                ctx.stop();
            }
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.ws_state
            .add_connection(self.id, self.client_type, ctx.address());
        log::info!(
            "WebSocket client {} connected ({:?})",
            self.id,
            self.client_type
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.ws_state.remove_connection(&self.id);
        log::info!("WebSocket client {} disconnected", self.id);
    }
}

impl Handler<BroadcastMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        if let Ok(json) = serde_json::to_string(&msg.0) {
            ctx.text(json);
        }
    }
}

impl Handler<SendPing> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, _msg: SendPing, ctx: &mut Self::Context) -> Self::Result {
        ctx.ping(b"")
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
                self.ws_state.update_pong_time(&self.id);
            }
            Ok(ws::Message::Text(text)) => {
                self.heartbeat = Instant::now();
                log::info!("WebSocket client {} sent text: {:?}", self.id, text);
            }
            Ok(ws::Message::Binary(_)) => {
                log::warn!("Binary messages not supported");
            }
            Ok(ws::Message::Close(reason)) => {
                log::info!("WebSocket client {} closing: {:?}", self.id, reason);
                ctx.stop();
            }
            Err(e) => {
                log::error!("WebSocket error for client {}: {:?}", self.id, e);
                ctx.stop();
            }
            _ => {}
        }
    }
}

// HTTP endpoints to upgrade to WebSocket
pub async fn get_public(
    req: HttpRequest,
    stream: web::Payload,
    ws_state: web::Data<WsState>,
) -> Result<HttpResponse> {
    let session = WebSocketSession::new(ClientType::Public, ws_state);
    ws::start(session, &req, stream)
}

pub async fn get_spymaster(
    req: HttpRequest,
    stream: web::Payload,
    ws_state: web::Data<WsState>,
) -> Result<HttpResponse> {
    let session = WebSocketSession::new(ClientType::Spymaster, ws_state);
    ws::start(session, &req, stream)
}

// Background task function
pub async fn websocket_cleanup_task(ws_state: web::Data<WsState>) {
    let mut interval = tokio::time::interval(HEARTBEAT);

    loop {
        interval.tick().await;
        ws_state.cleanup_and_ping().await;

        let count = ws_state.connection_count();
        if count > 0 {
            log::debug!("Active WebSocket connections: {}", count);
        }
    }
}
