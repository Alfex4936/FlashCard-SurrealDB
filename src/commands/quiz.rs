// src/commands/quiz.rs
use crate::model::{card::Card, db::*};
use crate::utils::input::*;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn start_quiz(client: Arc<Surreal<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    let cards: Vec<Card> = get_all_cards(&client).await?;

    if cards.is_empty() {
        println!("No cards found. Add some cards first.");
        return Ok(());
    }

    for card in cards {
        println!("Question: {}", card.question);
        let user_answer = get_input("Your answer: ");
        if user_answer.to_lowercase() == card.answer.to_lowercase() {
            println!("Correct!");
        } else {
            println!("! Incorrect. The correct answer is: {}", card.answer);
        }
    }

    Ok(())
}
