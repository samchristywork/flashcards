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

fn grade_answer(guess: &str, answer: &str) -> bool {
    println!(
        "{}Guess:{} {}",
        color(SHADE, SHADE, 255),
        color_reset(),
        guess
    );
    println!(
        "{}Answer:{} {}",
        color(SHADE, SHADE, 255),
        color_reset(),
        answer
    );
    speak(answer);
    println!(
        "{}Was it correct? (y/n){}",
        color(SHADE, SHADE, 255),
        color_reset()
    );

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.trim() == "y"
}

fn pick_random_card(cards: Vec<&Flashcard>, num: usize) -> Vec<&Flashcard> {
    let mut rng = rand::thread_rng();
    let mut picked = Vec::new();
    for _ in 0..num {
        let card = cards.choose(&mut rng).unwrap();
        picked.push(*card);
    }
    picked
}

fn append_to_file(filename: &str, line: String) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Can't open file");
    file.write_all(line.as_bytes())
        .expect("Can't write to file");
}

fn mark_correct(card: &Flashcard, filename: &str) {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    let line = format!(
        "{}	{}	{}	{}	{}\n",
        date, "correct", card.category, card.front, card.back,
    );

    append_to_file(filename, line);
}

fn mark_incorrect(card: &Flashcard, filename: &str) {
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    let line = format!(
        "{}	{}	{}	{}	{}\n",
        date, "incorrect", card.category, card.front, card.back
    );

    append_to_file(filename, line);
}

fn speak(text: &str) {
    let mut cmd = std::process::Command::new("espeak");
    cmd.arg(text);
    cmd.spawn().expect("Failed to speak");
}

fn display_flashcard(card: &Flashcard) {
    println!("{}: {}", card.category, card.front);
    speak(&card.front);
}

fn get_guess() -> String {
    let mut rl = DefaultEditor::new().unwrap();
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => line,
        Err(err) => err.to_string(),
    }
}

fn evaluate_guess(guess: String, card: &Flashcard) -> bool {
    let is_correct = grade_answer(guess.trim(), &card.back);
    is_correct
}

fn update_scorecard(card: &Flashcard, is_correct: bool, filename: &str) {
    if is_correct {
        mark_correct(&card, filename);
    } else {
        mark_incorrect(&card, filename);
    }
}

fn print_results(correct: usize, incorrect: usize) {
    println!("Correct: {}, Incorrect: {}", correct, incorrect);
}

fn administer_quiz(cards: Vec<&Flashcard>, filename: &str) {
    let num = ask_how_many_cards();
    let mut correct = 0;
    let mut incorrect = 0;
    for card in pick_random_card(cards, num) {
        display_flashcard(&card);
        let guess = get_guess();
        let is_correct = evaluate_guess(guess, &card);
        update_scorecard(&card, is_correct, filename);
        if is_correct {
            correct += 1;
        } else {
            incorrect += 1;
        }
    }
    print_results(correct, incorrect);
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}

fn parse_lines(contents: String) -> Vec<LogEntry> {
    let mut entries: Vec<LogEntry> = Vec::new();
    for line in contents.lines() {
        let mut fields = line.split('\t');
        let entry = LogEntry {
            date: strip_string(fields.next().unwrap().to_string()),
            result: strip_string(fields.next().unwrap().to_string()),
            card: Flashcard {
                category: strip_string(fields.next().unwrap().to_string()),
                front: strip_string(fields.next().unwrap().to_string()),
                back: strip_string(fields.next().unwrap().to_string()),
            },
        };
        entries.push(entry);
    }
    entries
}

fn count_correct(entries: &Vec<LogEntry>) -> usize {
    entries
        .iter()
        .filter(|entry| entry.result == "correct")
        .count()
}

fn days_since_last_quiz(entries: &Vec<LogEntry>) -> i64 {
    let last_date = &entries.last().unwrap().date;
    let last_date = chrono::NaiveDate::parse_from_str(last_date, "%Y-%m-%d %H:%M:%S").unwrap();
    let now = chrono::Local::now().naive_local().date();
    (now - last_date).num_days()
}

fn print_summary(filename: &str) {
    let contents = read_file(filename);
    let entries = parse_lines(contents);
    let correct = count_correct(&entries);
    let incorrect = entries.len() - correct;

    println!("Total Entries: {}", entries.len());
    println!("Correct: {} ({}%)", correct, correct * 100 / entries.len());
    println!(
        "Incorrect: {} ({}%)",
        incorrect,
        incorrect * 100 / entries.len()
    );
    println!(
        "Last Quiz: {} ({} days)",
        entries.last().unwrap().date,
        days_since_last_quiz(&entries)
    );
}

fn print_usage() {
    println!("Usage: flashcard <command>");
    println!("Commands:");
    println!("  show");
    println!("  quiz");
    println!("  summary");
    println!("  edit");
    println!("  help");
}

fn ask_how_many_cards() -> usize {
    print!("{}Quiz length:{} ", color(SHADE, SHADE, 255), color_reset());
    flush_stdout();

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.trim().parse().expect("Not a number")
}

fn main() {
    let cards_file = "/home/sam/.flashcard.cards";
    let log_file = "/home/sam/.flashcard.log";

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        print_usage();
        std::process::exit(1);
    }

    let cards = read_card_file(cards_file);
    let cards: Vec<&Flashcard> = cards.iter().collect();

    let command = &args[1];
    if command == "show" {
        show_cards(cards);
    } else if command == "quiz" {
        administer_quiz(cards, log_file);
    } else if command == "summary" {
        print_summary(log_file);
    } else if command == "edit" {
        match std::env::var("EDITOR") {
            Ok(editor) => {
                std::process::Command::new(editor)
                    .arg(cards_file)
                    .arg(log_file)
                    .status()
                    .expect("Failed to open editor");
            }
            Err(_) => {
                println!("Please set the EDITOR environment variable");
                std::process::exit(1);
            }
        }
    } else if command == "help" {
        print_usage();
    } else {
        print_usage();
        std::process::exit(1);
    }
}
