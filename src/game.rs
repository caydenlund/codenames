use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::words::get_words;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Team {
    Blue,
    Red,
    Neutral,
    Assassin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub word: String,
    pub team: Team,
    pub revealed: bool,
}

impl Card {
    pub fn public_json(&self) -> serde_json::Value {
        if self.revealed {
            serde_json::json!({
                "word": self.word,
                "team": self.team
            })
        } else {
            serde_json::json!({
                "word": self.word,
                "team": "unknown"
            })
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Turn {
    Blue,
    Red,
}

pub type Board = [[Card; 5]; 5];

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Arc<Mutex<Board>>,
    pub first_turn: Arc<Mutex<Turn>>,
}

impl GameState {
    pub fn new_board(first_turn: Turn) -> Board {
        let mut rng = rng();

        let mut teams = {
            let (blue_cards, red_cards) = match first_turn {
                Turn::Blue => (9, 8),
                Turn::Red => (8, 9),
            };
            let mut teams: Vec<_> = vec![Team::Blue; blue_cards]
                .into_iter()
                .chain(vec![Team::Red; red_cards])
                .chain(vec![Team::Assassin; 1])
                .chain(vec![Team::Neutral; 25 - red_cards - blue_cards - 1])
                .collect();
            teams.shuffle(&mut rng);
            teams.into_iter()
        };

        let mut words = get_words().into_iter();
        std::array::from_fn(|_| {
            std::array::from_fn(|_| Card {
                word: words.next().unwrap().to_string(),
                team: teams.next().unwrap(),
                revealed: false,
            })
        })
    }

    pub fn new_game(&self) {
        let new_first = match *self.first_turn.lock().unwrap() {
            Turn::Blue => Turn::Red,
            Turn::Red => Turn::Blue,
        };
        *self.first_turn.lock().unwrap() = new_first;

        *self.board.lock().unwrap() = Self::new_board(*self.first_turn.lock().unwrap());
    }

    pub fn new(first_turn: Turn) -> Self {
        GameState {
            board: Arc::new(Mutex::new(Self::new_board(first_turn))),
            first_turn: Arc::new(Mutex::new(first_turn)),
        }
    }

    pub fn public_json(&self) -> serde_json::Value {
        serde_json::json!(
            self.board
                .lock()
                .unwrap()
                .iter()
                .map(|row| row.iter().map(Card::public_json).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
    }

    pub fn spymaster_json(&self) -> serde_json::Value {
        serde_json::json!(*self.board.lock().unwrap())
    }

    pub fn reveal_card(&self, row: usize, col: usize) -> Card {
        self.board.lock().unwrap()[row][col].revealed = true;
        self.board.lock().unwrap()[row][col].clone()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(Turn::Blue)
    }
}
