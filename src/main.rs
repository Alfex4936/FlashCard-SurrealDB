// src/main.rs
use crate::commands::{
    add::{add_card, add_cards_from_file},
    delete::{delete_card, reset_db},
    edit::edit_card,
    list::list_cards,
    play::play_cards,
    quiz::start_quiz,
};

use crate::model::db::init_db_client;

use clap::Parser;
use std::sync::Arc;
use tokio;

mod commands;
mod model;
mod utils;

#[derive(Parser, Debug)]
#[clap(
    version,
    author = "Seok Won <ikr@kakao.com>",
    about = "A CLI flashcard app using SurrealDB"
)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Add a new flashcard
    Add { path: Option<String> },
    /// Edit an existing flashcard with id
    Edit { id: String },
    /// CAUTION: Delete all existing cards
    Reset,
    /// Delete a flashcard with id
    Delete { id: String },
    /// List all flashcards
    List,
    /// Start a quiz session
    Quiz,
    /// Shuffle all flashcards and play through them one by one
    Play,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let client = Arc::new(init_db_client().await?);

    match opts.command {
        Command::Add { path } => match path {
            Some(path) => add_cards_from_file(client.clone(), &path).await?,
            _ => {
                let _ = add_card(client.clone()).await?;
            }
        },
        Command::Edit { id } => {
            edit_card(client.clone(), &id).await?;
        }
        Command::Delete { id } => {
            delete_card(client.clone(), &id).await?;
        }
        Command::Reset => {
            reset_db(client.clone()).await?;
        }
        Command::List => {
            list_cards(client.clone()).await?;
        }
        Command::Quiz => {
            start_quiz(client.clone()).await?;
        }
        Command::Play => {
            play_cards(client.clone()).await?;
        }
    }

    Ok(())
}
