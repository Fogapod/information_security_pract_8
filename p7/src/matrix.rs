pub struct MatrixDimensions {
    pub height: usize,
    pub width: usize,
}

impl MatrixDimensions {
    pub fn is_square(&self) -> bool {
        self.height == self.width
    }
}

pub trait Matrix<T> {
    fn get_matrix(&self) -> &Vec<Vec<T>>;

    fn dimesions(&self) -> MatrixDimensions {
        let matrix = self.get_matrix();

        let height = matrix.len();
        let width = if height == 0 { 0 } else { matrix[0].len() };

        MatrixDimensions {
            height: height,
            width: width,
        }
    }
}
