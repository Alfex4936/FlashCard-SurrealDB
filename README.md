# Flashcard Master <img src="https://user-images.githubusercontent.com/2356749/234233512-7a3ecf91-3c70-4e2e-b83f-5571e63cabb3.png" style="height: 32px"></img>

### Clap-rs + SurrealDB-rs + tokio's async

FlashcardMaster is a command-line application built with Rust

that allows users to create, edit, delete, list, and quiz themselves on flashcards.

It uses SurrealDB as a backend to store flashcards for quick retrieval and modification.

Hosted SurrelDB server with [fly.io](https://surrealdb.com/docs/deployment/fly)

## Project Structure

```yaml
FlashcardMaster
├── Cargo.toml (dependencies and metadata for the Rust project)
├── .env (stores necessary environment variables such as SurrealDB credentials)
├── src
│   ├── main.rs (main CLI entry point, command handling, and app logic)
│   ├── commands
│   │   ├── add.rs (handles the 'add' command for creating new flashcards)
│   │   ├── edit.rs (handles the 'edit' command for updating existing flashcards)
│   │   ├── delete.rs (handles the 'delete' command for removing flashcards)
│   │   ├── list.rs (handles the 'list' command for displaying all flashcards)
│   │   ├── play.rs (handles the 'play' command for displaying all flashcards in console)
│   │   └── quiz.rs (handles the 'quiz' command for quizzing users with flashcards)
│   ├── model
│   │   ├── card.rs (defines the Card struct and its associated methods, used with SurrealDB)
│   │   └── db.rs (provides database connection and CRUD operations for flashcards)
│   └── utils
│       ├── input.rs (provides input handling and validation functions)
│       └── output.rs (provides formatting and printing functions for console output)
└── tests
    └── integration_tests.rs (tests for the main app logic)
```

## Installation

1. Clone the repository:

```bash
git clone https://github.com/Alfex4936/FlashCard-SurrealDB
```

2. Change to the project directory:

```bash
cd FlashCard-SurrealDB
```

3. Install the dependencies:

```bash
cargo build
```

4. Create a `.env` file in the project root with your SurrealDB credentials:

```bash
SURREALDB_DB_NS=namespace
SURREALDB_DB_DB=db_name
SURREALDB_DB_USER=username
SURREALDB_DB_PW=password
FLYIO_HOST=your.fly.dev
```

5. Run the application:

```bash
cargo run
```

## Usage

1. Add a flashcard:

from terminal or comma seperated file (Q,A) like `test.txt`.

![add](https://user-images.githubusercontent.com/2356749/234228019-29a6d062-ea09-4098-9535-ba53b9655c2f.png)

![add_from_file](https://user-images.githubusercontent.com/2356749/234229771-204c1137-7918-4eac-acaf-d8e1f78be817.png)


```bash
cargo run -- add

or

cargo run -- add test.txt
```

2. Edit a flashcard:

![edit](https://user-images.githubusercontent.com/2356749/234229336-a1bec801-211c-4339-a810-26ee1d0f2063.png)

```bash
cargo run -- edit [card_id]
```

3. Delete a flashcard:

```bash
cargo run -- delete [card_id]
```

4. List all flashcards:

![list](https://user-images.githubusercontent.com/2356749/234230547-a699d807-569f-4777-9b61-3b96194b46d5.png)

```bash
cargo run -- list
```

5. Start a quiz:

![quiz](https://user-images.githubusercontent.com/2356749/234231211-2db495f9-099b-4718-894f-942e8ac7ffee.gif)

```bash
cargo run -- quiz
```

1. Play all flashcards (shuffled):

![play](https://user-images.githubusercontent.com/2356749/234231207-da0718b3-b83f-4439-a986-ffa4edc18fb5.gif)

```bash
cargo run -- quiz
```

## License

This project is licensed under the MIT License.
