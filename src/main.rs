use rand::prelude::*;
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::prelude::*;

struct Flashcard {
    category: String,
    front: String,
    back: String,
}

fn color_reset() -> String {
    "\x1b[0m".to_string()
}

fn flush_stdout() {
    std::io::stdout().flush().expect("Failed to flush stdout");
}

fn strip_string(s: String) -> String {
    s.trim().to_string()
}

fn read_card_file(filename: &str) -> Vec<Flashcard> {
    let mut cards = Vec::new();
    let mut file = File::open(filename).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    for line in contents.lines() {
        let mut fields = line.split('\t');
        let card = Flashcard {
            category: strip_string(fields.next().unwrap().to_string()),
            front: strip_string(fields.next().unwrap().to_string()),
            back: strip_string(fields.next().unwrap().to_string()),
        };
        cards.push(card);
    }
    cards
}

fn fixed_width(s: String, width: usize) -> String {
    let mut s = s;
    while s.len() < width {
        s.push(' ');
    }
    s
}

fn color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

fn get_max_length(cards: &[&Flashcard], f: fn(&Flashcard) -> &str) -> usize {
    cards.iter().map(|card| f(card).len()).max().unwrap_or(0)
}

fn main() {
    let cards_file = "/home/sam/.flashcard.cards";

    let cards = read_card_file(cards_file);
    let cards: Vec<&Flashcard> = cards.iter().collect();
}
