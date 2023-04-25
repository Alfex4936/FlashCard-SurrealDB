// src/commands/list.rs
use crate::model::{card::Card, db::*};
use crate::utils::output::*;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn list_cards(client: Arc<Surreal<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    let cards: Vec<Card> = get_all_cards(&client).await?;

    print_cards(&cards);
    Ok(())
}
