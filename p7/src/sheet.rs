use crate::matrix::{Matrix, MatrixDimensions};

use std::fmt;

pub enum MirrorDirection {
    Vertical,
    Horizontal,
}

pub struct Sheet {
    matrix: Vec<Vec<bool>>,

    pub num_holes: usize,
}

impl Sheet {
    pub fn new(matrix: Vec<Vec<bool>>) -> Self {
        let num_holes = matrix
            .iter()
            .map(|row: &Vec<bool>| row.iter().filter(|value| **value).count())
            .sum();

        let instance = Self {
            matrix: matrix,
            num_holes: num_holes,
        };

        let dimesions = instance.dimesions();
        if !dimesions.is_square() {
            panic!("Bad sheet: not a square");
        }

        if dimesions.width != num_holes {
            panic!(format!(
                "Bad sheet: holes count should be equal to side length ({})",
                dimesions.width
            ));
        }

        instance
    }

    pub fn mirror(&mut self, direction: MirrorDirection) {
        let MatrixDimensions { height, width } = self.dimesions();

        let mut mirror = vec![vec![true; width]; height];

        match direction {
            MirrorDirection::Vertical => {
                for row in 0..height {
                    mirror[height - row - 1] = self.matrix[row].to_owned();
                }
            }
            MirrorDirection::Horizontal => {
                for row in 0..height {
                    for col in 0..width {
                        mirror[row][width - col - 1] = self.matrix[row][col];
                    }
                }
            }
        }

        self.matrix = mirror
    }
}

impl Matrix<bool> for Sheet {
    fn get_matrix(&self) -> &Vec<Vec<bool>> {
        &self.matrix
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MatrixDimensions { height, width } = self.dimesions();

        for row in 0..height {
            write!(f, "\n[")?;
            for col in 0..width {
                write!(f, "{}", if self.matrix[row][col] { " " } else { "#" })?;
            }
            write!(f, "]")?;
        }

        Ok(())
    }
}
