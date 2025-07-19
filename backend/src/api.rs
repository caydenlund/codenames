use crate::{
    game::GameState,
    websocket::{CardRevealData, WsMessage, WsState},
};
use actix_web::{HttpResponse, Responder, body::BoxBody, web};
use serde::Deserialize;

pub async fn get_board_public(game_state: web::Data<GameState>) -> impl Responder {
    web::Json(game_state.public_json())
}

pub async fn get_board_spymaster(game_state: web::Data<GameState>) -> impl Responder {
    web::Json(game_state.spymaster_json())
}

#[derive(Debug, Deserialize)]
pub struct RevealParams {
    pub row: usize,
    pub col: usize,
}

pub async fn post_reveal(
    req: web::Json<RevealParams>,
    game_state: web::Data<GameState>,
    ws_state: web::Data<WsState>,
) -> impl Responder {
    let (row, col) = (req.row, req.col);
    if row >= 5 || col >= 5 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid coordinates",
            "message": "Row and column must be between 0 and 4."
        }));
    }

    ws_state.broadcast(WsMessage::CardRevealed {
        data: CardRevealData {
            row,
            col,
            new_card_state: serde_json::json!(game_state.reveal_card(row, col)),
        },
    });

    HttpResponse::Ok().into()
}

pub async fn post_new_game(
    game_state: web::Data<GameState>,
    ws_state: web::Data<WsState>,
) -> impl Responder {
    game_state.new_game();

    ws_state.broadcast(WsMessage::GameReset {
        data: game_state.public_json(),
    });

    std::convert::Into::<HttpResponse<BoxBody>>::into(HttpResponse::Ok())
}
