// src/commands/play.rs
use crate::model::card::Card;
use crate::model::db::get_all_cards;
use rand::prelude::SliceRandom;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tokio::time::{sleep, Duration};

pub async fn play_cards(client: Arc<Surreal<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    let cards = get_all_cards(&client).await?;

    if cards.is_empty() {
        println!("No cards found. Please add some flashcards before playing.");
        return Ok(());
    }

    let mut rng = rand::thread_rng();
    let mut shuffled_cards: Vec<Card> = cards.into_iter().collect();
    shuffled_cards.shuffle(&mut rng);

    for card in shuffled_cards {
        print!("\x1B[2J\x1B[1;1H"); // Clear console
        println!("Question: {}", card.question);
        sleep(Duration::from_secs(3)).await;

        print!("\x1B[2J\x1B[1;1H"); // Clear console
        println!("Answer: {}", card.answer);
        sleep(Duration::from_secs(3)).await;
    }
    print!("\x1B[2J\x1B[1;1H"); // Clear console

    Ok(())
}
