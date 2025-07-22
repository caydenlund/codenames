use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use clap::Parser;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init();

    let cors = || {
        // FIXME
        // Cors::default()
        //     .allowed_origin_fn(|origin, _| origin.as_bytes().starts_with(b"http://localhost"))
        Cors::permissive()
    };

    let game_state = web::Data::new(game::GameState::default());
    let ws_state = web::Data::new(websocket::WsState::new());

    HttpServer::new(move || {
        let api = web::scope("/api")
            .wrap(cors())
            .route("/board/public", web::get().to(api::get_board_public))
            .route("/board/spymaster", web::get().to(api::get_board_spymaster))
            .route("/reveal", web::post().to(api::post_reveal))
            .route("/new_game", web::post().to(api::post_new_game));
        App::new()
            .app_data(game_state.clone())
            .app_data(ws_state.clone())
            .wrap(cors())
            .service(api)
            .route("/ws", web::get().to(websocket::get_ws))
            .route("/{path:.*}", web::get().to(frontend::get_frontend))
    })
    .bind((args.host.clone(), args.port))
    .inspect(|_| {
        println!("Codenames running at http://{}:{}", args.host, args.port);
    })?
    .run()
    .await
}
