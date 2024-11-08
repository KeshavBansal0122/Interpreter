mod token_type;
mod scanner;

use crate::scanner::{scan_tokens, ParsingError};
use crate::token_type::{Token, TokenType};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            // writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                match scan_tokens(file_contents) {
                    Ok(tok) => {print_braces(&tok);},
                    Err(err) => print_errors(&err),
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn print_braces(tokens: &Vec<Token>) {
    for token in tokens {
        match token.token_type {
            TokenType::LeftParen => println!("LEFT_PAREN ( null"),
            TokenType::RightParen => println!("RIGHT_PAREN ) null"),
            TokenType::LeftBrace => println!("LEFT_BRACE {{ null"),
            TokenType::RightBrace => println!("RIGHT_BRACE }} null"),
            TokenType::Eof => println!("EOF  null"),
            _ => {}
        }
    }
}
fn print_errors(errors: &Vec<ParsingError>) {
    for error in errors {
        let line = match error {
            ParsingError::UnexpectedEOF(line) => { format!("Unexpected EOF at {line}")},
            ParsingError::UnexpectedChar(char, line) => { format!("Unexpected character {char} at {line}")},
        };
        writeln!(io::stderr(), "{}", line).unwrap();
    }
}