// "Fox" Cipher decoder

const FOX_ENCODED: &'static str = "ШФССОЬОАИМЕЕТНВИИРИПЛЗВНЕПРСАОК ";

fn decode_fox_string(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();

    if chars.len() % 2 != 0 {
        panic!("Text length should be even for fox cipher");
    }

    let mut result = String::new();

    let left = &chars[..chars.len() / 2];
    let right = &chars[chars.len() / 2..];

    for i in 0..left.len() {
        result.push(left[i]);
        result.push(right[i]);
    }

    result
}

fn main() {
    println!("Fox cipher dexcoded: {}", decode_fox_string(FOX_ENCODED));
}
