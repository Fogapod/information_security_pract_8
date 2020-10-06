use p7::{read_text, sheet, write_text};

pub fn main() {
    let text = "ШИФРРЕШЕТКАЯВЛЯЕТСЯЧАСТНЫМСЛУЧАЕМШИФРАМАРШРУТНОЙПЕРЕСТАНОВКИ";
    let mut sheet = sheet::Sheet::new(vec![
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 1, 1, 0, 0, 1],
    ]);
    println!("Encoding: {}{}", text, sheet);

    let flips = [
        sheet::FlipDirection::DiagonalLR,
        sheet::FlipDirection::Horizontal,
        sheet::FlipDirection::DiagonalLR,
        sheet::FlipDirection::Horizontal,
    ];

    let paper = write_text(&mut sheet, &flips, text);

    sheet.reset_rotation();

    println!("Encoded:{}", paper);
    println!("Decoded: {}", read_text(&mut sheet, &flips, &paper));
}
