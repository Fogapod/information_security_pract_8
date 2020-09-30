// Vigenere Cipher implementation

use std::io::{self, Write};

const ALPHABET: &'static str = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя";

fn prepare_text(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace(|c: char| !ALPHABET.contains(c), "")
}

fn encode_text(text: &str, key: &str) -> String {
    let alphabet_chars: Vec<char> = ALPHABET.chars().collect();
    let alphabet_size = alphabet_chars.len();

    let key_chars: Vec<char> = key.chars().collect();
    let key_size = key_chars.len();

    let mut result = String::new();

    for (i, c) in text.chars().enumerate() {
        let text_char_index = ALPHABET.chars().position(|ch| ch == c).unwrap();
        let key_char_index = ALPHABET
            .chars()
            .position(|ch| ch == key_chars[i % key_size])
            .unwrap();

        let replacement_index = (text_char_index + key_char_index) % alphabet_size;

        result.push(alphabet_chars[replacement_index]);
    }

    result
}

fn decode_text(text: &str, key: &str) -> String {
    let alphabet_chars: Vec<char> = ALPHABET.chars().collect();
    let alphabet_size = alphabet_chars.len();

    let key_chars: Vec<char> = key.chars().collect();
    let key_size = key_chars.len();

    let mut result = String::new();

    for (i, c) in text.chars().enumerate() {
        let text_char_index = ALPHABET.chars().position(|ch| ch == c).unwrap();
        let key_char_index = ALPHABET
            .chars()
            .position(|ch| ch == key_chars[i % key_size])
            .unwrap();

        let replacement_index = (text_char_index + alphabet_size - key_char_index) % alphabet_size;

        result.push(alphabet_chars[replacement_index]);
    }

    result
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        print!("Enter secret key: ");
        io::stdout().flush()?;
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            println!("Error reading stdin: {}", e);
            continue;
        }

        break;
    }

    let secret_key = prepare_text(&buffer);

    loop {
        println!("Using secret key: {}", secret_key);

        print!("Enter text to encode: ");
        io::stdout().flush()?;
        buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            println!("Error reading stdin: {}", e);
            continue;
        }

        let prepared = prepare_text(&mut buffer);
        if prepared.len() == 0 {
            println!("Empty string!");
            continue;
        }

        let encoded = encode_text(&prepared, &secret_key);
        let decoded = decode_text(&encoded, &secret_key);

        println!(
            "Encoded {:indent$} => {}",
            prepared,
            encoded,
            indent = encoded.chars().count()
        );
        println!("Decoded {} => {}", encoded, decoded);
    }
}
