# Veyra — Rust Implementation

A tiny Rust implementation of the revolutionary Veyra language.

## Project structure

```text
veyra/
├── Cargo.toml
└── src/
    └── main.rs
```

## `Cargo.toml`

```toml
[package]
name = "veyra"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## `src/main.rs`

```rust
use std::collections::HashMap;

const NUMBERS: [&str; 10] = [
    "ka", "ve", "tri", "nox", "sai",
    "lum", "dra", "kei", "vor", "zen",
];

fn number_to_veyra(n: usize) -> Option<&'static str> {
    NUMBERS.get(n.checked_sub(1)?).copied()
}

fn veyra_to_number(word: &str) -> Option<usize> {
    NUMBERS.iter().position(|&x| x == word).map(|x| x + 1)
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

fn translate_numbers(input: &str) -> String {
    tokenize(input)
        .into_iter()
        .map(|word| {
            if let Some(number) = veyra_to_number(&word) {
                number.to_string()
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn number_table() -> HashMap<&'static str, usize> {
    NUMBERS
        .iter()
        .enumerate()
        .map(|(i, &word)| (word, i + 1))
        .collect()
}

fn main() {
    println!("=== VEYRA ===");
    println!("The revolutionary language of controlled chaos.\n");

    println!("Counting to 10:");

    for n in 1..=10 {
        let word = number_to_veyra(n).unwrap();
        println!("{n:>2} -> {word}");
    }

    println!("\nExample:");

    let message = "Veyra ka! Sai lum, vor zen. Dra kei nox!";
    println!("Veyra:      {message}");

    println!(
        "Tokenized:  {:?}",
        tokenize(message)
    );

    println!(
        "Translated: {}",
        translate_numbers(message)
    );

    println!("\nReverse lookup:");

    let numbers = number_table();

    for word in ["ka", "sai", "zen"] {
        match numbers.get(word) {
            Some(number) => println!("{word} = {number}"),
            None => println!("{word} = ???"),
        }
    }
}
```

## Running it

```bash
cargo run
```

Example output:

```text
=== VEYRA ===
The revolutionary language of controlled chaos.

Counting to 10:
 1 -> ka
 2 -> ve
 3 -> tri
 4 -> nox
 5 -> sai
 6 -> lum
 7 -> dra
 8 -> kei
 9 -> vor
10 -> zen

Example:
Veyra:      Veyra ka! Sai lum, vor zen. Dra kei nox!
Tokenized:  ["veyra", "ka", "sai", "lum", "vor", "zen", "dra", "kei", "nox"]
Translated: veyra 1 5 6 9 10 7 8 4

Reverse lookup:
ka = 1
sai = 5
zen = 10
```

## What's next?

The natural next step is turning Veyra into a **real interpreter** with:

```text
lexer → parser → AST → evaluator
```

Then we can give Veyra actual variables, arithmetic, conditionals, functions, and—most importantly—an unnecessarily dramatic error message system.
