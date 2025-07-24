use actix_cors::Cors;
use actix_web::web;
use clap::Parser;

#[cfg(not(feature = "shuttle"))]
use actix_web::{App, HttpServer};
#[cfg(feature = "shuttle")]
use shuttle_actix_web::ShuttleActixWeb;

mod api;
mod frontend;
mod game;
mod public;
mod websocket;
mod words;

#[derive(Parser, Debug)]
/// A web application implementation of Codenames
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Host address to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

fn config(
    cfg: &mut web::ServiceConfig,
    game_state: web::Data<game::GameState>,
    ws_state: web::Data<websocket::WsState>,
) {
    let cors = || {
        // TODO: only run permissively if this is a debug build
        Cors::permissive()
    };

    let api = web::scope("/api")
        .wrap(cors())
        .route("/board/public", web::get().to(api::get_board_public))
        .route("/board/spymaster", web::get().to(api::get_board_spymaster))
        .route("/reveal", web::post().to(api::post_reveal))
        .route("/new_game", web::post().to(api::post_new_game));

    let ws = web::scope("/ws")
        .wrap(cors())
        .route("/public", web::get().to(websocket::get_public))
        .route("/spymaster", web::get().to(websocket::get_spymaster));

    cfg.app_data(game_state)
        .app_data(ws_state)
        .service(api)
        .service(ws)
        .route("/{path:.*}", web::get().to(frontend::get_frontend));
}

#[cfg(not(feature = "shuttle"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init();

    let game_state = web::Data::new(game::GameState::default());
    let ws_state = web::Data::new(websocket::WsState::new());

    let cleanup_ws_state = ws_state.clone();
    tokio::spawn(async move {
        websocket::websocket_cleanup_task(cleanup_ws_state).await;
    });

    HttpServer::new(move || {
        App::new().configure(|cfg| config(cfg, game_state.clone(), ws_state.clone()))
    })
    .bind((args.host.clone(), args.port))
    .inspect(|_| {
        println!("Codenames running at http://{}:{}", args.host, args.port);
    })?
    .run()
    .await
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let game_state = web::Data::new(game::GameState::default());
    let ws_state = web::Data::new(websocket::WsState::new());

    let cleanup_ws_state = ws_state.clone();
    tokio::spawn(async move {
        websocket::websocket_cleanup_task(cleanup_ws_state).await;
    });

    Ok(shuttle_actix_web::ActixWebService(
        move |cfg: &mut web::ServiceConfig| {
            config(cfg, game_state, ws_state);
        },
    ))
}
