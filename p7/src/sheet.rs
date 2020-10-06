use crate::matrix::{Matrix, MatrixDimensions};

use std::fmt;

pub enum FlipDirection {
    Vertical,
    Horizontal,
    DiagonalLR, // can be represented by Vertical + Horizontal
                // DiagonalRL, // not possible to represent with other rotations
}

pub struct Sheet {
    matrix: Vec<Vec<u8>>,

    // needed for resetting
    flipped_v: bool,
    flipped_h: bool,

    pub num_holes: usize,
}

impl Sheet {
    pub fn new(matrix: Vec<Vec<u8>>) -> Self {
        let num_holes = matrix
            .iter()
            .map(|row: &Vec<u8>| row.iter().filter(|value| **value != 0).count())
            .sum();

        let instance = Self {
            matrix,
            num_holes,
            flipped_v: false,
            flipped_h: false,
        };

        let dimesions = instance.dimensions();
        let area = dimesions.area();

        if area % 2 != 0 {
            panic!(format!("Bad sheet: area should be even, got {}", area));
        }

        if area / num_holes != 4 {
            panic!(format!(
                "Bad sheet: holes count should be 1/4 of sheet area ({}), got {}",
                area / 4,
                num_holes,
            ));
        }

        instance
    }

    pub fn hole_at(&self, row: usize, column: usize) -> bool {
        self.matrix[row][column] != 0
    }

    pub fn flip(&mut self, direction: &FlipDirection) {
        let MatrixDimensions { height, width } = self.dimensions();

        let mut mirror = vec![vec![0; width]; height];

        match *direction {
            FlipDirection::Vertical => {
                for row in 0..height {
                    mirror[height - row - 1] = self.matrix[row].to_owned();
                }

                self.flipped_v = !self.flipped_v;
            }
            FlipDirection::Horizontal => {
                for (row_index, row) in mirror.iter_mut().enumerate().take(height) {
                    for col in 0..width {
                        row[width - col - 1] = self.matrix[row_index][col];
                    }
                }

                self.flipped_h = !self.flipped_h;
            }
            FlipDirection::DiagonalLR => {
                for row in 0..height {
                    for col in 0..width {
                        mirror[height - row - 1][width - col - 1] = self.matrix[row][col];
                    }
                }

                self.flipped_v = !self.flipped_v;
                self.flipped_h = !self.flipped_h;
            }
        }

        self.matrix = mirror
    }

    pub fn reset_rotation(&mut self) {
        if self.flipped_v {
            self.flip(&FlipDirection::Vertical);
        }

        if self.flipped_h {
            self.flip(&FlipDirection::Horizontal);
        }
    }
}

impl Matrix<u8> for Sheet {
    fn get_matrix(&self) -> &Vec<Vec<u8>> {
        &self.matrix
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MatrixDimensions { height, width } = self.dimensions();

        for row in 0..height {
            write!(f, "\n[")?;
            for col in 0..width {
                write!(f, "{}", if self.matrix[row][col] != 0 { " " } else { "#" })?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
