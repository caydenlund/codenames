use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

mod api;
mod game;
mod public;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cors = || {
        Cors::default()
            .allowed_origin_fn(|origin, _| origin.as_bytes().starts_with(b"http://localhost"))
    };

    let game_state = web::Data::new(game::GameState::default());
    let ws_state = web::Data::new(websocket::WsState::new());

    HttpServer::new(move || {
        let api = web::scope("/api")
            .wrap(cors())
            .route("/board/public", web::get().to(api::get_board_public))
            .route("/board/spymaster", web::get().to(api::get_board_spymaster))
            .route("/reveal", web::post().to(api::post_reveal));
        App::new()
            .app_data(game_state.clone())
            .app_data(ws_state.clone())
            .wrap(cors())
            .service(api)
            .route("/ws", web::get().to(websocket::get_ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
