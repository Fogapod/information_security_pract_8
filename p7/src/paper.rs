use crate::matrix::{Matrix, MatrixDimensions};
use crate::sheet::Sheet;

use std::fmt;

pub struct Paper {
    matrix: Vec<Vec<char>>,
}

impl Paper {
    pub fn new(dimesions: MatrixDimensions) -> Result<Self, String> {
        let area = dimesions.area();

        if area % 2 != 0 {
            return Err(format!("Bad paper: area should be even, got {}", area));
        }

        let MatrixDimensions { height, width } = dimesions;

        Ok(Self {
            matrix: vec![vec![' '; width]; height],
        })
    }

    pub fn draw(&mut self, sheet: &Sheet, chars: &[char]) -> usize {
        let MatrixDimensions { height, width } = self.dimensions();

        let mut current_char = 0;

        for row in 0..height {
            for col in 0..width {
                if sheet.hole_at(row, col) {
                    self.matrix[row][col] = chars[current_char];
                    current_char += 1;
                }
            }
        }

        current_char
    }

    pub fn read(&self, sheet: &Sheet) -> Vec<char> {
        let MatrixDimensions { height, width } = self.dimensions();

        let mut chars = vec![];

        for row in 0..height {
            for col in 0..width {
                if sheet.hole_at(row, col) {
                    chars.push(self.matrix[row][col]);
                }
            }
        }

        chars
    }
}

impl Matrix<char> for Paper {
    fn get_matrix(&self) -> &Vec<Vec<char>> {
        &self.matrix
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MatrixDimensions { height, width } = self.dimensions();

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
