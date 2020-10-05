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

    sheet.reset();
    let paper = write_text(&mut sheet, text);

    println!("Encoded:{}", paper);
    println!("Decoded: {}", read_text(&mut sheet, &paper));
}
