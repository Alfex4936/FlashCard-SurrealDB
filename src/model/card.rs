// src/model/card.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub card_id: String,
    pub question: String,
    pub answer: String,
}

impl Card {
    pub fn new(card_id: String, question: String, answer: String) -> Self {
        Card {
            card_id,
            question,
            answer,
        }
    }
}
