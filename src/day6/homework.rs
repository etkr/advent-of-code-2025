use crate::day6::problem::Problem;
use ndarray::Array2;
use std::error::Error;
use std::str::FromStr;

struct Homework {
    problems: Vec<Problem>,
}

impl Homework {
    fn total(&self) -> i32 {
        self.problems.iter().map(Problem::result).sum()
    }
}

impl FromStr for Homework {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_of_vec: Vec<Vec<&str>> = s
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();

        let ndarray: Array2<&str> = Array2::from(vec_of_vec);
        let transposed = ndarray.t();
        let problems = ndarray.rows().into_iter().map(|row| {});

        return Homework {};
    }
}
