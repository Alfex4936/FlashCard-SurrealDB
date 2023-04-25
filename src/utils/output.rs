// src/utils/output.rs
use crate::model::card::Card;

pub fn print_cards(cards: &[Card]) {
    if cards.is_empty() {
        println!("No cards found.");
    } else {
        for card in cards {
            println!("{:#?}", card);
            println!("ID: {}", card.card_id);
            println!("Question: {}", card.question);
            println!("Answer: {}", card.answer);
            println!("------------------------------------");
        }
    }
}
