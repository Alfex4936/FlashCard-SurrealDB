// src/commands/delete.rs
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn delete_card(
    client: Arc<Surreal<Client>>,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    crate::model::db::delete_card(&client, id).await?;
    println!("Card deleted successfully.");
    Ok(())
}

pub async fn reset_db(client: Arc<Surreal<Client>>) -> Result<(), Box<dyn std::error::Error>> {
    client.query("DELETE FROM cards").await?;
    println!("All cards deleted successfully.");
    Ok(())
}
