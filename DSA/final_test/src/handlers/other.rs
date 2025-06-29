use crate::string_algos::rabin_karp::rabin_karp;
use std::io;

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BOLD: &str = "\x1b[1m";

pub fn handle_other_algorithms_menu() {
    loop {
        println!("\n{}{}String Algorithms:{}", BOLD, CYAN, RESET);
        println!("1. Rabin-Karp String Matching");
        println!("0. Back to Main Menu");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_rabin_karp(),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET),
        }
    }
}

fn handle_rabin_karp() {
    println!("\n{}{}Rabin-Karp String Matching Algorithm{}", BOLD, CYAN, RESET);

    println!("Enter the text to search in:");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    let text = text.trim();

    println!("Enter the pattern to search for:");
    let mut pattern = String::new();
    io::stdin()
        .read_line(&mut pattern)
        .expect("Failed to read line");
    let pattern = pattern.trim();

    let matches = rabin_karp(pattern, text);

    if matches.is_empty() {
        println!("{}No matches found.{}", YELLOW, RESET);
    } else {
        println!("{}Found {} matches at positions:{}", GREEN, matches.len(), RESET);
        for (i, pos) in matches.iter().enumerate() {
            println!("  Match {}: position {}", i + 1, pos);
        }

        println!("\n{}Text with matches:{}", CYAN, RESET);
        let mut highlighted_text = String::new();
        let pattern_len = pattern.len();

        for (i, c) in text.chars().enumerate() {
            if matches.iter().any(|&pos| i >= pos && i < pos + pattern_len) {
                highlighted_text.push_str(&format!("{}{}{}", GREEN, c, RESET));
            } else {
                highlighted_text.push(c);
            }
        }

        println!("{}", highlighted_text);
    }
}