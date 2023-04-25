// src/commands/edit.rs
use crate::model::{card::Card, db::*};
use crate::utils::input::*;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn edit_card(
    client: Arc<Surreal<Client>>,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let card_to_edit: Card = get_card(&client, id).await?;

    let new_question = get_input_with_default("Enter the new question: ", &card_to_edit.question);
    let new_answer = get_input_with_default("Enter the new answer: ", &card_to_edit.answer);

    let updated_card = Card::new(card_to_edit.card_id, new_question, new_answer);
    update_card(&client, &updated_card).await?;

    println!("Card updated successfully.");
    Ok(())
}
