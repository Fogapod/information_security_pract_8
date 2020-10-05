use crate::matrix::{Matrix, MatrixDimensions};

use std::fmt;

pub enum MirrorDirection {
    Vertical,
    Horizontal,
}

pub struct Sheet {
    matrix: Vec<Vec<u8>>,

    // needed for resetting
    mirrored_v: bool,
    mirrored_h: bool,

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
            mirrored_v: false,
            mirrored_h: false,
        };

        let dimesions = instance.dimesions();
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

    pub fn mirror(&mut self, direction: MirrorDirection) {
        let MatrixDimensions { height, width } = self.dimesions();

        let mut mirror = vec![vec![0; width]; height];

        match direction {
            MirrorDirection::Vertical => {
                for row in 0..height {
                    mirror[height - row - 1] = self.matrix[row].to_owned();
                }

                self.mirrored_v = !self.mirrored_v;
            }
            MirrorDirection::Horizontal => {
                for (row_index, row) in mirror.iter_mut().enumerate().take(height) {
                    for col in 0..width {
                        row[width - col - 1] = self.matrix[row_index][col];
                    }
                }

                self.mirrored_h = !self.mirrored_h;
            }
        }

        self.matrix = mirror
    }

    pub fn reset(&mut self) {
        if self.mirrored_v {
            self.mirror(MirrorDirection::Vertical);
        }

        if self.mirrored_h {
            self.mirror(MirrorDirection::Horizontal);
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
        let MatrixDimensions { height, width } = self.dimesions();

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
