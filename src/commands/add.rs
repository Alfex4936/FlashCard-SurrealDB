// src/commands/add.rs
use crate::model::{card::Card, db::*};
use crate::utils::input::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn add_card(client: Arc<Surreal<Client>>) -> Result<Card, Box<dyn std::error::Error>> {
    let question = get_input("Enter the question: ");
    let answer = get_input("Enter the answer: ");
    let card: Card = create_card(&client, &question, &answer).await?;
    println!("Card added successfully with ID: {}", card.card_id);
    Ok(card)
}

pub async fn add_cards_from_file(
    client: Arc<Surreal<Client>>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, ',');

        if let (Some(question), Some(answer)) = (parts.next(), parts.next()) {
            let card: Card = create_card(&client, question, answer).await?;
            println!("Card added successfully with ID: {}", card.card_id);
        } else {
            println!("Invalid line format: {}", line);
        }
    }

    Ok(())
}

pub async fn add_card_from_str(
    client: Arc<Surreal<Client>>,
    question: &str,
    answer: &str,
) -> Result<Card, Box<dyn std::error::Error>> {
    let card: Card = create_card(&client, question, answer).await?;
    println!("Card added successfully with ID: {}", card.card_id);
    Ok(card)
}
