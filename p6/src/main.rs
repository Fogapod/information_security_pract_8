/// "Replacement" Cipher implementation
///
/// # What?
/// Algo used here:
/// Input:
///     key: a sequence of numbers from 1 to X, in random order.
///     text: a string, character count should be devisible by key.
///
/// Output: encoded text.
///
/// Encoder:
///     - Puts string into matrix with the width of key length going lr-tb.
///     - Reads matrix rows in order defined by key from top to bottom and appends characters into resulting string.
///
/// Decoder: Same steps as encoder but in reverse.
///
/// Example:
///     key: "21"    -> [2, 1]
///     text: "3456" -> [["3", "4"], ["5", "6"]]
///
///     key        :  2 1
///     text matrix: [3 4]
///                  [5 6]
///                   | |
///                   | \/
///                   | 46 35 -> "4635"
///                   `----/\
use std::iter::FromIterator;

pub struct MatrixDimensions {
    width: usize,
    height: usize,
}
/// The only reason struct is used here is because I rewritten this program 5 times and I no longer remember why
pub struct Secret {
    text_chars: Vec<char>,
    key: Vec<usize>,
}

/// Converts key in form of string into vector of integers
/// # Panics
/// If key is invalid. e.g. `"124"` or `"456"`
fn key_to_vec(key: &str) -> Vec<usize> {
    let key: Vec<usize> = key
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut sorted_key = key.to_vec();
    sorted_key.sort();

    let mut i = 0;
    while i < sorted_key.len() - 1 {
        if i == 0 && sorted_key[i] != 1 {
            panic!(
                "Invalid key: lowest value should be 1, got {}",
                sorted_key[i]
            );
        }

        if sorted_key[i] + 1 != sorted_key[i + 1] {
            panic!(
                "Invalid key: {} follows {}, expected {} instead",
                sorted_key[i + 1],
                sorted_key[i],
                sorted_key[i] + 1,
            );
        }

        i += 1;
    }

    key
}

impl Secret {
    pub fn new(text: &str, key: &str) -> Self {
        let chars: Vec<char> = text.chars().collect();

        let key = key_to_vec(key);

        // panic early
        Self::get_matrix_dimensions(chars.len(), key.len());

        Secret {
            text_chars: chars,
            key: key,
        }
    }

    /// Calculates matrix dimensions required to contain given text length
    /// # Panics
    ///   - If text is shorter than key
    ///   - If text is not divisible by key
    fn get_matrix_dimensions(text_len: usize, key_len: usize) -> MatrixDimensions {
        if key_len > text_len {
            panic!("Invalid text length: should be greater than matrix length");
        }

        let remainder = text_len % key_len;
        if remainder != 0 {
            panic!("Invalid text length: should be dividable by key length");
        }

        MatrixDimensions {
            width: key_len,
            height: text_len / key_len,
        }
    }

    fn cipher(&self, decode: bool, rounds: u32) -> String {
        if rounds == 0 {
            return String::from_iter(self.text_chars.iter());
        }

        let MatrixDimensions { width, height } =
            Self::get_matrix_dimensions(self.text_chars.len(), self.key.len());

        // Needed in case of multiple rounds
        //
        // Macro here because Rust cannot understand first if statement in loop body yet
        #[allow(unused_assignments)]
        let mut chars = vec![];

        let mut result = vec![];

        for round in 0..rounds {
            if round == 0 {
                chars = self.text_chars.to_owned();
            } else {
                chars = result.to_owned();
            }

            result = vec![' '; self.text_chars.len()];

            for (i, col_index) in self.key.iter().enumerate() {
                for row_index in 0..height {
                    // Here we are iterating columns in the order defined by key.
                    // At the same time we keep track of our position on original string
                    // using `i` and `row_index` indexes (since `row_index` isnt altered
                    // it can be reused).
                    // Resulting vector is being filled using offsets.
                    //
                    // Note the `col_index - 1`: this is because key starts from 1 not 0
                    let from_index = (col_index - 1) * height + row_index;
                    let to_index = row_index * width + i;

                    if decode {
                        result[from_index] = chars[to_index];
                    } else {
                        result[to_index] = chars[from_index];
                    }
                }
            }
        }
        return String::from_iter(result);
    }

    /// Encodes secret
    pub fn encode(&self, rounds: u32) -> String {
        self.cipher(false, rounds)
    }

    /// Decodes secret
    pub fn decode(&self, rounds: u32) -> String {
        self.cipher(true, rounds)
    }
}

fn main() {
    let text = "ПЕРЕСТАНОВКАТЕКСТАПОСТОЛБЦАМ";
    let key = "4312567";
    let rounds = 1;

    let secret = Secret::new(text, key);

    let encoded = secret.encode(rounds);
    let encoded_secret = Secret::new(&encoded, key);

    let decoded = encoded_secret.decode(rounds);

    println!("Encoded: {}", encoded);
    println!("Decoded: {}", decoded);
}
