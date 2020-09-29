// Polybius Square Cipher encoder/decoder

use phf::{phf_map, Map};
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

const SUBSTITUTIONS: Map<&'static str, &'static str> = phf_map! {
    "i" => "j"
};

const SQUARE_SIDE: u32 = 5;

fn prepare_string(s: &str) -> String {
    let mut s = s.trim().to_lowercase();

    // replace all non-ASCII characters
    s = s.replace(|c: char| !c.is_ascii_alphabetic(), "");

    // apply substitutions
    for (k, v) in SUBSTITUTIONS.entries() {
        s = s.replace(k, v);
    }

    s
}

fn encode_string(s: &str) -> String {
    let mut result = String::from("");

    let reference_index = 'a' as u32;
    let i_index = 'i' as u32 - reference_index;

    for c in s.chars() {
        let mut index = c as u32 - reference_index;
        if index > i_index {
            index -= 1;
        }

        let x = index % 5 + 1;
        let y = index / SQUARE_SIDE + 1;

        result.push_str(&format!("{}{}", x, y));
        //println!("{} {} xy: {} {}", c, index, x, y);
    }

    result
}

fn decode_string(s: &str) -> String {
    if s.len() % 2 != 0 {
        panic!("Bad string length ({})", s.len());
    }
    let mut result = String::from("");

    fn next_to_ascii(chars: &mut Peekable<Chars>) -> u32 {
        let next = chars.next().unwrap();

        next.to_digit(10).expect(&format!("Not a digit: {}", next))
    }

    let reference_index = 'a' as u8;
    let i_index = 'i' as u8 - reference_index;

    let mut chars = s.chars().peekable();
    while chars.peek().is_some() {
        let x = next_to_ascii(&mut chars);
        let y = next_to_ascii(&mut chars);

        let mut index = ((x - 1) + (y - 1) * SQUARE_SIDE) as u8;
        if index > i_index {
            index += 1;
        }

        result.push((index + reference_index) as char);
    }

    result
}

fn main() -> io::Result<()> {
    loop {
        print!("Enter text to encode: ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            println!("Error reading stdin: {}", e);
            continue;
        }

        let prepared = prepare_string(&mut buffer);
        if prepared.len() == 0 {
            println!("Empty string!");
            continue;
        }

        let encoded = encode_string(&prepared);
        let decoded = decode_string(&encoded);

        println!(
            "Encoded {:indent$} => {}",
            prepared,
            encoded,
            indent = encoded.len()
        );
        println!("Decoded {} => {}", encoded, decoded);
    }
}
