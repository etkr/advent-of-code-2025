pub enum Operation {
    Add,
    Multiply,
}

pub struct Problem {
    terms: Vec<i32>,
    operation: Operation,
}

impl Problem {
    pub fn new(terms: Vec<i32>, operation: Operation) -> Self {
        Self { terms, operation }
    }
    pub fn result(&self) -> i32 {
        let initial_value = match self.operation {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };
        self.terms
            .iter()
            .fold(initial_value, |acc, &term| match self.operation {
                Operation::Add => acc + term,
                Operation::Multiply => acc * term,
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::problem::{Operation, Problem};

    #[test]
    fn problem() {
        // 123 * 45 * 6 = 33210
        let problem = Problem::new(vec![123, 45, 6], Operation::Multiply);
        let result = problem.result();
        assert_eq!(33210, result);
    }
}
