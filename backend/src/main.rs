use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/api/board")]
async fn board() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cors = || {
        Cors::default()
            .allowed_origin_fn(|origin, _| origin.as_bytes().starts_with(b"http://localhost"))
    };
    HttpServer::new(move || App::new().wrap(cors()).service(board))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
