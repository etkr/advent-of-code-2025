#[derive(Debug)]
pub struct MonotonicStack<T> {
    target_capacity: usize,
    internal_stack: Vec<T>,
    input_size: usize,
    current: usize,
}

impl<T: Copy + PartialOrd> MonotonicStack<T> {
    pub fn push(&mut self, value: T) {
        if self.internal_stack.is_empty() {
            self.internal_stack.push(value);
            self.current += 1;
        } else if self.items_left() <= self.space_left() {
            self.internal_stack.push(value);
            self.current += 1;
        } else if let Some(&last_value) = self.internal_stack.last()
            && value > last_value
        {
            self.internal_stack.pop();
            self.internal_stack.push(value);
            self.current += 1;
        } else if self.internal_stack.len() >= self.target_capacity {
            self.current += 1;
        } else if let Some(&last_value) = self.internal_stack.last()
            && value <= last_value
        {
            self.internal_stack.push(value);
            self.current += 1;
        }
    }
    fn space_left(&self) -> usize {
        self.target_capacity - self.internal_stack.len()
    }

    fn items_left(&self) -> usize {
        self.input_size - self.current
    }

    pub fn values(&self) -> &[T] {
        &self.internal_stack[..]
    }

    pub fn new(target_capacity: usize, input_size: usize) -> Self {
        Self {
            target_capacity,
            input_size,
            internal_stack: Vec::with_capacity(target_capacity),
            current: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::monotonic_stack::MonotonicStack;

    #[test]
    fn test_monotonic_stack_insert() {
        let input: Vec<u32> = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        let mut monotonic_stack = MonotonicStack::new(12, input.len());
        for value in input {
            monotonic_stack.push(value);
        }
        assert_eq!(
            vec![4, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            monotonic_stack.values()
        );
    }

    #[test]
    fn test_monotonic_stack_insert2() {
        let input: Vec<u32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let mut monotonic_stack = MonotonicStack::new(12, input.len());
        for value in input {
            monotonic_stack.push(value);
        }
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1],
            monotonic_stack.values()
        );
    }

    #[test]
    fn test_monotonic_stack_insert3() {
        let input: Vec<u32> = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        let mut monotonic_stack = MonotonicStack::new(12, input.len());
        for value in input {
            monotonic_stack.push(value);
        }
        assert_eq!(
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            monotonic_stack.values()
        );
    }
}
