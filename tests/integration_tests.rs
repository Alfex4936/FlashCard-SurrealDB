// tests/integration_tests.rs
use flashcard_master::{
    model::db::{delete_card, get_all_cards, update_card},
    *,
};

use std::sync::Arc;
use tokio;

#[tokio::test]
async fn test_create_and_delete_card() {
    let client = Arc::new(init_db_client().await.unwrap());
    let question = "What is the capital of France?";
    let answer = "Paris";
    let card: Card = add_card_from_str(client.clone(), question, answer)
        .await
        .unwrap();
    assert_eq!(card.question, question);
    assert_eq!(card.answer, answer);

    delete_card(&client.clone(), &card.card_id).await.unwrap();
    let all_cards = get_all_cards(&client.clone()).await.unwrap();
    assert!(!all_cards.iter().any(|c| c.card_id == card.card_id));
}

#[tokio::test]
async fn test_update_card() {
    let client = Arc::new(init_db_client().await.unwrap());
    let question = "What is the capital of Spain?";
    let answer = "Madrid";
    let card: Card = add_card_from_str(client.clone(), question, answer)
        .await
        .unwrap();

    let new_question = "What is the capital of Italy?";
    let new_answer = "Rome";
    let updated_card = Card::new(
        card.card_id.clone(),
        new_question.to_string(),
        new_answer.to_string(),
    );
    update_card(&client, &updated_card).await.unwrap();

    let all_cards = get_all_cards(&client).await.unwrap();
    let fetched_card = all_cards
        .iter()
        .find(|c| c.card_id == card.card_id)
        .unwrap();
    assert_eq!(fetched_card.question, new_question);
    assert_eq!(fetched_card.answer, new_answer);

    delete_card(&client, &card.card_id).await.unwrap();
}
