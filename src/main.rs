use rand::prelude::*;
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::prelude::*;

struct Flashcard {
    category: String,
    front: String,
    back: String,
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

fn main() {
    let cards_file = "/home/sam/.flashcard.cards";

    let cards = read_card_file(cards_file);
    let cards: Vec<&Flashcard> = cards.iter().collect();
}
