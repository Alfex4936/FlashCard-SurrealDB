// src/utils/input.rs
use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_input_with_default(prompt: &str, default_value: &str) -> String {
    let input = get_input(&format!("{} (default: {}): ", prompt, default_value));

    if input.is_empty() {
        default_value.to_string()
    } else {
        input
    }
}
