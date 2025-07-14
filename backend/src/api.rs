use crate::game::Game;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use std::sync::Mutex;

pub mod board {
    use super::*;

    pub async fn public(data: web::Data<Mutex<Game>>) -> impl Responder {
        web::Json(data.lock().unwrap().public_json())
    }

    pub async fn spymaster(data: web::Data<Mutex<Game>>) -> impl Responder {
        web::Json(data.lock().unwrap().spymaster_json())
    }
}

#[derive(Debug, Deserialize)]
pub struct RevealParams {
    row: usize,
    col: usize,
}

pub async fn reveal(
    data: web::Data<Mutex<Game>>,
    params: web::Query<RevealParams>,
) -> impl Responder {
    if params.row >= 5 || params.col >= 5 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid coordinates",
            "message": "Row and column must be between 0 and 4"
        }));
    }

    data.lock().unwrap().reveal_card(params.row, params.col);

    HttpResponse::Ok().json(data.lock().unwrap().public_json())
}
