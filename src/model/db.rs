// src/model/db.rs
use crate::model::card::Card;
use dotenv::dotenv;
use std::env;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Wss;
use surrealdb::opt::auth::Root;
use surrealdb::Result as DbResult;
use surrealdb::Surreal;

use blake3::hash;
use rand::thread_rng;
use rand::Rng;
use serde_json::Value;

pub async fn init_db_client() -> DbResult<Surreal<Client>> {
    dotenv().ok();
    let host = env::var("FLYIO_HOST").expect("FLYIO_HOST must be set");
    let user = env::var("SURREALDB_DB_USER").expect("SURREALDB_DB_USER must be set");
    let pw = env::var("SURREALDB_DB_PW").expect("SURREALDB_DB_PW must be set");
    let ns = env::var("SURREALDB_DB_NS").expect("SURREALDB_DB_NS must be set");
    let db_name = env::var("SURREALDB_DB_DB").expect("SURREALDB_DB_DB must be set");

    // Connect to the server
    let db = Surreal::new::<Wss>(host).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &user,
        password: &pw,
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns(ns).use_db(db_name).await?;

    // db.query("DELETE FROM cards").await?;

    Ok(db)
}

pub async fn create_card(client: &Surreal<Client>, question: &str, answer: &str) -> DbResult<Card> {
    let hash_input = format!("{}{}", question, answer);
    let digest = hash(hash_input.as_bytes());
    let hash_string = digest.to_hex().as_str().to_owned();

    // Use a shorter, unique hash by selecting a random substring from the full hash
    let hash_length = 7;
    let full_hash_chars: Vec<char> = hash_string.chars().collect();
    let start_index = thread_rng().gen_range(0..(full_hash_chars.len() - hash_length));
    let short_hash: String = full_hash_chars[start_index..start_index + hash_length]
        .iter()
        .collect();

    let new_card = Card::new(short_hash, question.to_string(), answer.to_string());
    // Change this line to receive a raw JSON Value instead of Card
    let created_card: Value = client.create("cards").content(&new_card).await?;

    // Manually construct the Card object from the JSON Value
    let id = created_card["card_id"].as_str().unwrap_or("").to_string();
    let question = created_card["question"].as_str().unwrap_or("").to_string();
    let answer = created_card["answer"].as_str().unwrap_or("").to_string();

    Ok(Card::new(id, question, answer))
}

pub async fn get_card(
    client: &Surreal<Client>,
    card_id: &str,
) -> Result<Card, Box<dyn std::error::Error>> {
    // Select a specific record from a table
    let mut response = client
        .query(&format!(
            "SELECT * FROM cards WHERE card_id = '{}'",
            card_id
        ))
        .await?;

    let card: Option<Card> = response.take(0)?;

    Ok(card.unwrap())
}

pub async fn get_all_cards(client: &Surreal<Client>) -> DbResult<Vec<Card>> {
    // Select a specific record from a table
    let cards = client.select("cards").await?;

    Ok(cards)
}

pub async fn update_card(client: &Surreal<Client>, card: &Card) -> DbResult<Card> {
    let updated_card: Card = client
        .update(("cards", &card.card_id))
        .content(card)
        .await?;
    Ok(updated_card)
}

pub async fn delete_card(client: &Surreal<Client>, id: &str) -> DbResult<()> {
    client
        .query(&format!("DELETE FROM cards WHERE card_id = '{}'", id))
        .await?;
    Ok(())
}
