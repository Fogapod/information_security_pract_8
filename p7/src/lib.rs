mod matrix;

pub mod paper;
pub mod sheet;

use paper::Paper;
use sheet::{FlipDirection, Sheet};
use std::iter::FromIterator;

use matrix::Matrix;

pub fn write_text(sheet: &mut Sheet, flips: &[FlipDirection], text: &str) -> Result<Paper, String> {
    let mut paper = Paper::new(sheet.dimensions())?;

    let chars: Vec<char> = text.chars().collect();
    let char_count = chars.len();

    if char_count != paper.dimensions().area() {
        return Err(format!(
            "Bad text: need exactly {} characters, got {}",
            paper.dimensions().area(),
            char_count
        ));
    }

    let mut current_char = 0;

    for flip in flips {
        paper.draw(sheet, &chars[current_char..current_char + sheet.num_holes]);
        current_char += sheet.num_holes;

        sheet.flip(flip);
    }

    Ok(paper)
}

pub fn read_text(sheet: &mut Sheet, flips: &[FlipDirection], paper: &Paper) -> String {
    let mut result: Vec<char> = vec![];

    for flip in flips.iter() {
        result.extend(paper.read(&sheet));
        sheet.flip(flip);
    }

    String::from_iter(result)
}
