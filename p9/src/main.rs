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

fn stdin_prompt(buffer: &mut String, prompt: &str) -> io::Result<()> {
    buffer.clear();

    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        if let Err(e) = io::stdin().read_line(buffer) {
            println!("Error reading stdin: {}", e);
            continue;
        }
        break;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let keys = vec![
        "щбёармгшняогусмышхьш",
        "ь ъкпдчюдзыжкцйфцкоё",
        "ечыбющжгюэелывпжп хо",
        "зяцъюёйсёрюжьяьзцфмй",
        "эюзалтрютичэйврфатпе",
        "кбсхэаьящпъюлщашмоке",
        "енвоаогщздйьъйтётщыы",
        "пчфдаяиацсдцвгяалркщ",
        "юбакхш  ечочоадшцкэя",
        "шенажхйту шчо гжиднй",
    ];

    for key in keys {
        println!("Using secret key: {}", key);

        loop {
            stdin_prompt(&mut buffer, "Send or recieve (1/0): ")?;
            match buffer.trim() {
                "1" => loop {
                    stdin_prompt(&mut buffer, "Enter text to send: ")?;
                    let prepared = prepare_text(&mut buffer);
                    if prepared.len() == 0 {
                        println!("Empty string!");
                        continue;
                    }

                    println!("Encoded: {}", encode_text(&prepared, &key));
                    break;
                },
                "0" => loop {
                    stdin_prompt(&mut buffer, "Enter recieved text: ")?;
                    let prepared = prepare_text(&mut buffer);
                    if prepared.len() == 0 {
                        println!("Empty string!");
                        continue;
                    }

                    println!("Decoded: {}", decode_text(&prepared, &key));
                    break;
                },
                _ => {
                    println!("Did not understand you");
                    continue;
                }
            }
            break;
        }
    }

    Ok(())
}
