use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct DoubleMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>
}

impl DoubleMatrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> DoubleMatrix {
        assert!(rows > 0 && cols > 0, "Expected {} rows and {} cols to be positive", rows, cols);
        assert!(data.len() == rows * cols, "len {} != {} rows * {} cols", data.len(), rows, cols);
        DoubleMatrix { rows, cols, data }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        i + self.rows * j
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[self.index(i, j)]
    }

    pub fn put(&mut self, i: usize, j: usize, value: f64) {
        let ind = self.index(i, j);
        self.data[ind] = value;
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn is_row_vector(&self) -> bool {
        self.rows == 1
    }

    pub fn is_column_vector(&self) -> bool {
        self.cols == 1
    }

    pub fn is_vector(&self) -> bool {
        self.is_row_vector() || self.is_column_vector()
    }

    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[self.index(i, j)] != self.data[self.index(j, i)] {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for DoubleMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[self.index(i, j)])?;
                if j < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, "; ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}
