use crate::matrix::{Matrix, MatrixDimensions};

use std::fmt;

pub struct Paper {
    matrix: Vec<Vec<char>>,
}

impl Paper {
    pub fn new(height: usize, width: usize) -> Self {
        let instance = Self {
            matrix: vec![vec![' '; width]; height],
        };

        let dimesions = instance.dimesions();
        if !dimesions.is_square() {
            panic!("Bad paper: not a square");
        }

        instance
    }
}

impl Matrix<char> for Paper {
    fn get_matrix(&self) -> &Vec<Vec<char>> {
        &self.matrix
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MatrixDimensions { height, width } = self.dimesions();

        for row in 0..height {
            write!(f, "\n[")?;
            for col in 0..width {
                write!(f, "{}", self.matrix[row][col])?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
