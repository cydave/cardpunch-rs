use std::collections::HashMap;
use std::fs::File;
use std::string::String;

use serde::{Deserialize};

type Column = [i32; 12];
type Row = Vec<char>;

#[derive(Debug, Deserialize)]
struct CharsetFile {
    on_char: char,
    off_char: char,
    charmap: HashMap<char, Column>
}


impl CharsetFile {
    fn load(path: &str) -> Self {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}

#[derive(Debug, Deserialize)]
struct Charset {
    on_char: char,
    off_char: char,
    enc_map: HashMap<char, Column>,
    dec_map: HashMap<Column, char>
}

impl Charset {

    fn from_file(path: &str) -> Self {
        let charset_file = CharsetFile::load(path);
        let enc_map = charset_file.charmap.to_owned();
        let dec_map = enc_map.iter().map(|(k, v)| (v.to_owned(), k.to_owned())).collect();
        Self { on_char: charset_file.on_char, off_char: charset_file.off_char, enc_map: charset_file.charmap.to_owned(), dec_map }
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

#[derive(Debug)]
struct Card {
    charset: Charset,
    columns: Vec<Column>,
}

impl Card {
    fn with_charset(charset: Charset) -> Self {
        Card { charset: charset, columns: Vec::new()  }
    }

    fn punch_char(&mut self, c: char) {
        let column = self.charset.encode(c);
        self.columns.push(column);
    }

    fn punch_str(&mut self, s: &str) {
        for c in s.chars() {
            self.punch_char(c);
        }
    }

    fn read(&self) -> String {
        self.columns.iter().map(|c| self.charset.decode(*c)).collect()
    }

    fn rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for i in 0..12 {
            let mut row = Row::new();
            for column in self.columns.iter() {
                let c: char = match column[i] {
                    1 => self.charset.on_char,
                    0 => self.charset.off_char,
                    _ => self.charset.on_char
                };
                row.push(c);
            }
            rows.push(row)
        }
        rows
    }

    fn print(&self) {
        let rows = self.rows();
        let card_width = self.columns.len()+1;
        println!("    {}", "_".repeat(card_width));
        println!("   /{}", self.read());
        println!("12/ {}", rows[0].iter().collect::<String>());
        println!("11|  {}", rows[1].iter().collect::<String>());
        for (index, row) in rows[2..].into_iter().enumerate() {
            println!(" {}|  {}", index.to_string(), row.into_iter().collect::<String>());
        }
        println!("  |__{}", "_".repeat(card_width));
    }
}

fn main() {
    let charset = Charset::from_file("./charsets/example.json");
    let mut card = Card::with_charset(charset);
    card.punch_str("RUST CARD PUNCH IS A THING NOW!!111 \\O/");
    card.print();
}
