use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

mod api;
mod game;
mod public;

use game::{Game, Turn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cors = || {
        Cors::default()
            .allowed_origin_fn(|origin, _| origin.as_bytes().starts_with(b"http://localhost"))
    };

    let game = web::Data::new(Mutex::new(Game::new(
        &[
            "a".into(),
            "b".into(),
            "c".into(),
            "d".into(),
            "e".into(),
            "f".into(),
            "g".into(),
            "h".into(),
            "i".into(),
            "j".into(),
            "k".into(),
            "l".into(),
            "m".into(),
            "n".into(),
            "o".into(),
            "p".into(),
            "q".into(),
            "r".into(),
            "s".into(),
            "t".into(),
            "u".into(),
            "v".into(),
            "w".into(),
            "x".into(),
            "y".into(),
        ],
        Turn::Blue,
    )));

    HttpServer::new(move || {
        let api_scope = web::scope("/api")
            .wrap(cors())
            .route("/board/public", web::get().to(api::board::public))
            .route("/board/spymaster", web::get().to(api::board::spymaster))
            .route("/reveal", web::post().to(api::reveal));
        App::new()
            .app_data(game.clone())
            .wrap(cors())
            .service(api_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
