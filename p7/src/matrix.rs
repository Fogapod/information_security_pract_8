pub struct MatrixDimensions {
    pub height: usize,
    pub width: usize,
}

impl MatrixDimensions {
    pub fn area(&self) -> usize {
        self.height * self.width
    }

    pub fn fits_string(&self, s: &str) -> bool {
        self.area() == s.chars().count()
    }
}

impl PartialEq for MatrixDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.width == other.width
    }
}
impl Eq for MatrixDimensions {}

pub trait Matrix<T> {
    fn get_matrix(&self) -> &Vec<Vec<T>>;

    fn dimensions(&self) -> MatrixDimensions {
        let matrix = self.get_matrix();

        let height = matrix.len();
        let width = if height == 0 { 0 } else { matrix[0].len() };

        MatrixDimensions { height, width }
    }
}
