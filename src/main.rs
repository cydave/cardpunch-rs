use std::collections::HashMap;
use std::fs::File;
use std::string::String;

use serde::Deserialize;

type Column = [i32; 12];
type Row = Vec<char>;

#[derive(Debug, Deserialize)]
struct CharsetFile {
    on_char: char,
    off_char: char,
    charmap: HashMap<char, Column>,
}

impl CharsetFile {
    fn load(path: &str) -> Self {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}

#[derive(Debug, Clone)]
struct Charset {
    on_char: char,
    off_char: char,
    enc_map: HashMap<char, Column>,
    dec_map: HashMap<Column, char>,
}

impl Charset {
    fn from_file(path: &str) -> Self {
        let charset_file = CharsetFile::load(path);
        let enc_map = charset_file.charmap.to_owned();
        let dec_map = enc_map
            .iter()
            .map(|(k, v)| (v.to_owned(), k.to_owned()))
            .collect();
        Self {
            on_char: charset_file.on_char,
            off_char: charset_file.off_char,
            enc_map,
            dec_map,
        }
    }

    fn encode(&self, k: char) -> Column {
        match self.enc_map.get(&k) {
            Some(v) => *v,
            None => {
                eprintln!("Charset does not contain {:?}", k);
                panic!();
            }
        }
    }

    fn decode(&self, v: Column) -> char {
        self.dec_map[&v].to_owned()
    }
}

struct PunchMachine {
    charset: Charset,
}

impl PunchMachine {
    fn new(charset: Charset) -> Self {
        Self { charset }
    }

    fn punch_char(&self, c: char, card: &mut Card) {
        let column = self.charset.encode(c);
        card.columns.push(column);
    }

    fn punch_str(&self, s: &str) -> Card {
        let mut card = Card::new(self.charset.to_owned());
        for c in s.chars() {
            self.punch_char(c, &mut card);
        }
        card
    }
}

#[derive(Debug)]
struct Card {
    charset: Charset,
    columns: Vec<Column>,
}

impl Card {
    fn new(charset: Charset) -> Self {
        Self {
            charset,
            columns: Vec::new(),
        }
    }

    fn read(&self) -> String {
        self.columns
            .iter()
            .map(|c| self.charset.decode(*c))
            .collect()
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for i in 0..12 {
            let mut row = Row::new();
            for column in self.columns.iter() {
                let c: char = match column[i] {
                    1 => self.charset.on_char,
                    _ => self.charset.off_char,
                };
                row.push(c);
            }
            rows.push(row)
        }
        rows
    }

    fn print(&self) {
        let rows = self.rows();
        let card_width = self.columns.len() + 1;
        println!("    {}", "_".repeat(card_width));
        println!("   /{}", self.read());
        println!("12/ {}", rows[0].iter().collect::<String>());
        println!("11| {}", rows[1].iter().collect::<String>());
        for (index, row) in rows[2..].iter().enumerate() {
            println!(" {}| {}", index, row.iter().collect::<String>());
        }
        println!("  |_{}", "_".repeat(card_width));
    }
}

fn main() {
    let charset = Charset::from_file("./charsets/example.json");
    let machine = PunchMachine::new(charset);
    let card = machine.punch_str("RUST CARD PUNCH IS A THING NOW!!111 \\O/");
    card.print();
}
