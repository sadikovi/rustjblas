extern crate rand;

use std::fmt::{Display, Error, Formatter};
use self::rand::Rng;

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

    // create matrix with capacity of rows * cols, it is up to client to add values to the matrix
    pub fn with_capacity(rows: usize, cols: usize) -> DoubleMatrix {
        assert!(rows > 0 && cols > 0, "Expected {} rows and {} cols to be positive", rows, cols);
        DoubleMatrix { rows, cols, data: Vec::with_capacity(rows * cols) }
    }

    pub fn rand(rows: usize, cols: usize) -> DoubleMatrix {
        assert!(rows > 0 && cols > 0, "Expected {} rows and {} cols to be positive", rows, cols);
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..data.capacity() {
            data.push(rng.next_f64());
        }
        DoubleMatrix { rows, cols, data: data }
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

    // replace data of the matrix with provided vector
    pub fn replace(&mut self, data: Vec<f64>) {
        assert!(
            data.len() == self.rows * self.cols,
            "len {} != {} rows * {} cols",
            data.len(), self.rows, self.cols
        );
        self.data = data;
    }

    pub fn add_scalar(&mut self, value: f64) {
        for ind in 0..self.data.len() {
            self.data[ind] += value;
        }
    }

    pub fn add_matrix(&mut self, matrix: &DoubleMatrix) {
        assert!(self.rows == matrix.rows && self.cols == matrix.cols);
        assert!(self.data.len() == matrix.data.len());
        for ind in 0..self.data.len() {
            self.data[ind] += matrix.data[ind];
        }
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

    // pretty print of the matrix
    pub fn show(&self, truncate: bool) {
        // convert each element into string and collect max width for each column
        let row_bound = 7;
        let col_bound = 3;

        let truncated = truncate && (self.rows > row_bound || self.cols > col_bound);

        let rows = if truncated { row_bound } else { self.rows };
        let cols = if truncated { col_bound } else { self.cols };

        let mut values = Vec::with_capacity(rows * cols);
        for col in 0..cols {
            for row in 0..rows {
                values.push(format!("{:.4}", self.data[self.index(row, col)]));
            }
        }

        // max widths for each column, each value is padded to this width
        let mut col_size = Vec::with_capacity(self.cols);
        for col in 0..cols {
            let mut max_width = 0;
            for row in 0..rows {
                let value = values[row + rows * col].len();
                if value > max_width {
                    max_width = value;
                }
            }
            col_size.push(max_width);
        }

        let mut matrix_str = String::new();
        matrix_str.push_str(&format!("Matrix {} x {}:\n", self.rows, self.cols));

        for i in 0..rows {
            for j in 0..cols {
                let value = &values[i + rows * j];
                matrix_str.push_str(&format!("{:width$} ", value, width = col_size[j]));
            }
            if truncated {
                matrix_str.push_str("...");
            }
            matrix_str.push('\n');
        }

        if truncated {
            matrix_str.push_str("... (truncated)");
        }

        println!("{}", matrix_str);
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
