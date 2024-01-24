use rand::prelude::*;
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::prelude::*;

const SHADE: u8 = 150;

struct Flashcard {
    category: String,
    front: String,
    back: String,
}

struct LogEntry {
    date: String,
    result: String,
    card: Flashcard,
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

fn print_header(category_len: usize, front_len: usize) {
    println!(
        "{}  {}  {}",
        fixed_width("Category".to_string(), category_len),
        fixed_width("Front".to_string(), front_len),
        "Back",
    );
}

fn print_card(
    card: &Flashcard,
    category_len: usize,
    front_len: usize,
    category_color: &str,
    front_color: &str,
    back_color: &str,
) {
    println!(
        "{}{}  {}{}  {}{}{}",
        category_color,
        fixed_width(card.category.to_string(), category_len),
        front_color,
        fixed_width(card.front.to_string(), front_len),
        back_color,
        card.back,
        color_reset()
    );
}

fn show_cards(cards: Vec<&Flashcard>) {
    let max_category_len = get_max_length(&cards, |card| &card.category);
    let max_front_len = get_max_length(&cards, |card| &card.front);

    let category_color = color(SHADE, 255, 255);
    let front_color = color(255, SHADE, 255);
    let back_color = color(255, 255, SHADE);

    print_header(max_category_len, max_front_len);

    for card in cards {
        print_card(
            card,
            max_category_len,
            max_front_len,
            &category_color,
            &front_color,
            &back_color,
        );
    }
}

fn main() {
    let cards_file = "/home/sam/.flashcard.cards";

    let cards = read_card_file(cards_file);
    let cards: Vec<&Flashcard> = cards.iter().collect();
}
