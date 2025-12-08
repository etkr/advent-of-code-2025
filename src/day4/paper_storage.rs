use crate::day4::matrix::Matrix;
use std::error::Error;
use std::str::FromStr;

pub struct PaperStorage {
    matrix: Matrix<char>,
}

impl PaperStorage {
    pub fn n_forklift_accessible_paper_rolls(&self) -> usize {
        self.matrix
            .adjacent_windows()
            .filter(|view| view.is_paper() && view.n_adjacent_paper_rolls() < 4)
            .count()
    }
}

impl FromStr for PaperStorage {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_of_vec: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let matrix = Matrix::new(vec_of_vec);
        Ok(PaperStorage { matrix })
    }
}
