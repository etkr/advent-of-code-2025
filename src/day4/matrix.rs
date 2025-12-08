pub struct Matrix<T> {
    cols: usize,
    rows: usize,
    data: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Self { cols, rows, data }
    }

    pub fn adjacent_windows(&self) -> impl Iterator<Item = View<T>> {
        AdjacentIterator {
            cols: self.cols,
            rows: self.rows,
            data: self.data.clone(),
            x: 0,
            y: 0,
            end: false,
        }
    }
}

struct AdjacentIterator<T> {
    cols: usize,
    rows: usize,
    data: Vec<Vec<T>>,
    x: usize,
    y: usize,
    end: bool,
}

impl<T: Copy> AdjacentIterator<T> {
    fn top_left(&self) -> Option<T> {
        if self.y > 0 && self.x > 0 {
            return Some(self.data[self.y - 1][self.x - 1]);
        }
        None
    }
    fn top(&self) -> Option<T> {
        if self.y > 0 {
            return Some(self.data[self.y - 1][self.x]);
        }
        None
    }
    fn top_right(&self) -> Option<T> {
        if self.y > 0 && self.x < self.cols - 1 {
            return Some(self.data[self.y - 1][self.x + 1]);
        }
        None
    }
    fn left(&self) -> Option<T> {
        if self.x > 0 {
            return Some(self.data[self.y][self.x - 1]);
        }
        None
    }
    fn right(&self) -> Option<T> {
        if self.x < self.cols - 1 {
            return Some(self.data[self.y][self.x + 1]);
        }
        None
    }

    fn bottom_right(&self) -> Option<&T> {
        if self.y < self.rows - 1 && self.x < self.cols - 1 {
            return Some(&self.data[self.y + 1][self.x + 1]);
        }
        None
    }
    fn bottom(&self) -> Option<&T> {
        if self.y < self.rows - 1 {
            return Some(&self.data[self.y + 1][self.x]);
        }
        None
    }
    fn bottom_left(&self) -> Option<T> {
        if self.y < self.rows - 1 && self.x > 0 {
            return Some(self.data[self.y + 1][self.x - 1]);
        }
        None
    }

    pub fn increment(&mut self) {
        if self.x >= self.cols - 1 && self.y >= self.rows - 1 {
            self.end = true;
            return;
        }
        if self.x >= self.cols - 1 {
            self.x = 0;
            self.y += 1;
            return;
        }
        self.x += 1
    }
}

impl<T> Iterator for AdjacentIterator<T> {
    type Item = View<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let mut adjacent = Vec::new();

        if let Some(item) = self.top_left() {
            adjacent.push(item);
        }
        if let Some(item) = self.top() {
            adjacent.push(item);
        }
        if let Some(item) = self.top_right() {
            adjacent.push(item);
        }
        if let Some(item) = self.left() {
            adjacent.push(item);
        }
        if let Some(item) = self.right() {
            adjacent.push(item);
        }
        if let Some(item) = self.bottom_left() {
            adjacent.push(item);
        }
        if let Some(item) = self.bottom() {
            adjacent.push(item);
        }
        if let Some(item) = self.bottom_right() {
            adjacent.push(item);
        }
        let element = self.data[self.y][self.x];
        self.increment();
        Some(View {
            element: &element,
            adjacent,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct View<'a, T> {
    element: &'a T,
    adjacent: Vec<T>,
}

impl<T> View<'_, T> {
    pub fn new(element: &T, adjacent: Vec<T>) -> Self {
        Self {
            element: e,
            adjacent,
        }
    }
}

impl View<char> {
    pub fn is_paper(&self) -> bool {
        self.element == '@'
    }

    pub fn n_adjacent_paper_rolls(&self) -> usize {
        self.adjacent.iter().filter(|x| **x == '@').count()
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::matrix::{Matrix, View};

    #[test]
    fn test_iterator() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let matrix = Matrix::new(data);
        let mut iter = matrix.adjacent_windows();
        assert_eq!(View::new(1, vec![2, 5, 6]), iter.next().unwrap());
        assert_eq!(View::new(2, vec![1, 3, 5, 6, 7]), iter.next().unwrap());
        assert_eq!(View::new(3, vec![2, 4, 6, 7, 8]), iter.next().unwrap());
        assert_eq!(View::new(4, vec![3, 7, 8]), iter.next().unwrap());
        assert_eq!(View::new(5, vec![1, 2, 6, 9, 10]), iter.next().unwrap());
        assert_eq!(
            View::new(6, vec![1, 2, 3, 5, 7, 9, 10, 11]),
            iter.next().unwrap()
        );
    }

    #[test]
    fn test_last_iterator() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let matrix = Matrix::new(data);
        let iter = matrix.adjacent_windows();
        let view = View::new(12, vec![7, 8, 11]);
        assert_eq!(view, iter.last().unwrap());
    }
}
