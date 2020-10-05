use p7::paper;
use p7::sheet;

pub fn main() {
    let mut sheet = sheet::Sheet::new(vec![
        vec![true, false, false],
        vec![true, true, false],
        vec![false, false, false],
    ]);
    println!("Sheet: {}", sheet);

    sheet.mirror(sheet::MirrorDirection::Horizontal);
    println!("Sheet hor: {}", sheet);

    sheet.mirror(sheet::MirrorDirection::Vertical);
    println!("Sheet ver: {}", sheet);

    let _paper = paper::Paper::new(1, 1);
}
