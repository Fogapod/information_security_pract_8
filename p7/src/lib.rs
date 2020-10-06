mod matrix;

pub mod paper;
pub mod sheet;

use paper::Paper;
use sheet::{FlipDirection, Sheet};
use std::iter::FromIterator;

use matrix::Matrix;

pub fn write_text(sheet: &mut Sheet, text: &str) -> Paper {
    let mut paper = Paper::new(sheet.dimesions());

    let chars: Vec<char> = text.chars().collect();
    let char_count = chars.len();

    paper.draw(sheet, &chars[..(char_count / 4)]);

    // 180 degrees for some reason
    sheet.flip(FlipDirection::Horizontal);
    sheet.flip(FlipDirection::Vertical);

    paper.draw(sheet, &chars[char_count / 4..char_count / 2]);

    sheet.flip(FlipDirection::Horizontal);

    paper.draw(sheet, &chars[char_count / 2..(char_count / 4) * 3]);

    // 180 degrees again (according to task..)
    sheet.flip(FlipDirection::Horizontal);
    sheet.flip(FlipDirection::Vertical);

    paper.draw(sheet, &chars[(char_count / 4) * 3..]);

    paper
}

pub fn read_text(sheet: &mut Sheet, paper: &Paper) -> String {
    let mut result: Vec<char> = vec![];

    sheet.flip(FlipDirection::Horizontal);

    result.extend(paper.read(&sheet));

    // 180 degrees for some reason
    sheet.flip(FlipDirection::Horizontal);
    sheet.flip(FlipDirection::Vertical);

    result.extend(paper.read(&sheet));

    sheet.flip(FlipDirection::Horizontal);

    result.extend(paper.read(&sheet));

    // // 180 degrees again (according to task..)
    sheet.flip(FlipDirection::Horizontal);
    sheet.flip(FlipDirection::Vertical);

    result.extend(paper.read(&sheet));

    String::from_iter(result)
}
