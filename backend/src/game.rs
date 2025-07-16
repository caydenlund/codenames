use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    pub fn new(word: &str) -> Self {
        Self {
            word: word.to_string(),
            team: Team::Neutral,
            revealed: false,
        }
    }

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

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Arc<Mutex<[[Card; 5]; 5]>>,
}

impl GameState {
    pub fn new(words: &[String; 25], first_turn: Turn) -> Self {
        let ((blue_cards, red_cards), assassin_cards) = (
            match first_turn {
                Turn::Blue => (9, 8),
                Turn::Red => (8, 9),
            },
            1,
        );

        let mut board =
            std::array::from_fn(|r| std::array::from_fn(|c| Card::new(&words[r * 5 + c])));

        let mut rng = rng();

        let mut indices: Vec<_> = (0..5).flat_map(|r| (0..5).map(move |c| (r, c))).collect();
        indices.shuffle(&mut rng);
        let mut indices = indices.into_iter();

        [
            (blue_cards, Team::Blue),
            (red_cards, Team::Red),
            (assassin_cards, Team::Assassin),
        ]
        .into_iter()
        .for_each(|(count, team)| {
            indices
                .by_ref()
                .take(count)
                .for_each(|(r, c)| board[r][c].team = team);
        });

        GameState {
            board: Arc::new(Mutex::new(board)),
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
        Self::new(
            &[
                "alpha".into(),
                "bravo".into(),
                "charlie".into(),
                "delta".into(),
                "echo".into(),
                "foxtrot".into(),
                "golf".into(),
                "hotel".into(),
                "india".into(),
                "juliett".into(),
                "kilo".into(),
                "lima".into(),
                "mike".into(),
                "november".into(),
                "oscar".into(),
                "papa".into(),
                "quebec".into(),
                "romeo".into(),
                "sierra".into(),
                "tango".into(),
                "uniform".into(),
                "victor".into(),
                "whiskey".into(),
                "x-ray".into(),
                "yankee".into(),
            ],
            Turn::Blue,
        )
    }
}
