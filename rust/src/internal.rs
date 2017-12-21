// Copyright (c) 2017 sadikovi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::cmp;
use std::fmt::{Display, Error, Formatter};
use rand::{Rng, weak_rng};

// Macro to generate elementwise operations
macro_rules! elementwise_scalar_op {
    ($fn_scalar_mut:ident, $fn_scalar:ident, $fn_matrix_mut:ident, $fn_matrix:ident, $op:tt) => (
        #[inline]
        pub fn $fn_scalar_mut(&mut self, value: f64) {
            for i in 0..self.data.len() {
                self.data[i] = self.data[i] $op value;
            }
        }

        pub fn $fn_scalar(&self, value: f64) -> Self {
            let mut clone = self.clone();
            clone.$fn_scalar_mut(value);
            clone
        }

        #[inline]
        pub fn $fn_matrix_mut(&mut self, other: &DoubleMatrix) {
            assert_eq!(self.shape(), other.shape(),
                "Matrices do not have the same shape ({:?} != {:?})", self.shape(), other.shape());
            for i in 0..self.data.len() {
                self.data[i] = self.data[i] $op other.data[i];
            }
        }

        pub fn $fn_matrix(&self, other: &DoubleMatrix) -> Self {
            let mut clone = self.clone();
            clone.$fn_matrix_mut(other);
            clone
        }
    )
}

// Strict representation of the double matrix with as little overhead as possible.
// This allows us to resolve library conflicts and implement transformations efficiently.
pub struct DoubleMatrix {
    rows: usize, // number of rows in this matrix
    cols: usize, // number of columns in this matrix
    data: Vec<f64> // data in column major order
}

impl DoubleMatrix {
    // Create new matrix from data
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len(),
            "Dimensions mismatch: {} * {} != {}", rows, cols, data.len());
        // use shrink_to_fit
        DoubleMatrix { rows: rows, cols: cols, data: data }
    }

    // Create new matrix of 1s
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, vec![1f64; rows * cols])
    }

    // Create new matrix of 0s
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols, vec![0f64; rows * cols])
    }

    // Generate matrix of random values
    // This method is faster than method in nalgebra crate
    pub fn new_random(rows: usize, cols: usize) -> Self {
        let mut rng = weak_rng();
        let data = rng.gen_iter::<f64>().take(rows * cols).collect::<Vec<f64>>();
        Self::new(rows, cols, data)
    }

    // Generate identity matrix that has 1s as main diagonal and the rest are 0s.
    // If matrix is not square then the largest upper square diagonal is 1s
    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut matrix = Self::zeros(rows, cols);
        let dim = cmp::min(rows, cols);
        for i in 0..dim {
            matrix.put(i, i, 1f64);
        }
        matrix
    }

    // Convert row and col indices into vector index, no boundary checking is performed
    #[inline]
    pub fn m2v(&self, row: usize, col: usize) -> usize {
        row + col * self.rows
    }

    // Convert vector index into row and col, no boundary checking is performed
    #[inline]
    pub fn v2m(&self, i: usize) -> (usize, usize) {
        (i % self.rows, i / self.rows)
    }

    // Return rows and columns as shape tuple
    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    // Return number of rows
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    // Return number of columns
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    // Return slice of data
    #[inline]
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    // Return mutable slice of data
    #[inline]
    pub fn data_mut(&mut self) -> &mut [f64] {
        &mut self.data
    }

    #[inline]
    pub fn put(&mut self, row: usize, col: usize, value: f64) {
        let i = self.m2v(row, col);
        self.data[i] = value;
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let i = self.m2v(row, col);
        self.data[i]
    }

    elementwise_scalar_op!(add_scalar_mut, add_scalar, add_matrix_mut, add_matrix, +);
    elementwise_scalar_op!(sub_scalar_mut, sub_scalar, sub_matrix_mut, sub_matrix, -);
    elementwise_scalar_op!(mul_scalar_mut, mul_scalar, mul_matrix_mut, mul_matrix, *);
    elementwise_scalar_op!(div_scalar_mut, div_scalar, div_matrix_mut, div_matrix, /);
}

impl Clone for DoubleMatrix {
    fn clone(&self) -> Self {
        DoubleMatrix::new(self.rows, self.cols, self.data.clone())
    }
}

impl Display for DoubleMatrix {
    // Copied from nalgebra
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn val_width(val: f64, f: &mut Formatter) -> usize {
            match f.precision() {
                Some(precision) => format!("{:.1$}", val, precision).chars().count(),
                None => format!("{}", val).chars().count()
            }
        }

        let (rows, cols) = self.shape();

        if rows == 0 || cols == 0 {
            return write!(f, "[ ]");
        }

        let mut max_length = 0;
        let mut lengths = vec![0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                let idx = self.m2v(i, j);
                lengths[idx] = val_width(self.data[idx], f);
                max_length = cmp::max(max_length, lengths[idx]);
            }
        }

        let max_length_with_space = max_length + 1;

        try!(writeln!(f, ""));
        try!(writeln!(f, "  ┌ {:>width$} ┐", "", width = max_length_with_space * cols - 1));

        for i in 0..rows {
            try!(write!(f, "  │"));
            for j in 0..cols {
                let idx = self.m2v(i, j);
                let number_length = lengths[idx] + 1;
                let pad = max_length_with_space - number_length;
                try!(write!(f, " {:>thepad$}", "", thepad = pad));
                match f.precision() {
                    Some(precision) => try!(write!(f, "{:.1$}", self.data[idx], precision)),
                    None => try!(write!(f, "{}", self.data[idx]))
                }
            }
            try!(writeln!(f, " │"));
        }

        try!(writeln!(f, "  └ {:>width$} ┘", "", width = max_length_with_space * cols - 1));
        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_index_conversion(rows: usize, cols: usize) {
        let matrix = DoubleMatrix::new_random(rows, cols);
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                assert_eq!(matrix.v2m(matrix.m2v(i, j)), (i, j))
            }
        }
    }

    #[test]
    fn test_index_conversions() {
        test_index_conversion(123, 57);
        test_index_conversion(57, 123);
        test_index_conversion(1, 123);
        test_index_conversion(123, 1);
        test_index_conversion(1, 1);
    }

    #[test]
    #[should_panic(expected = "Dimensions mismatch: 3 * 2 != 100")]
    fn test_new_dim_mismatch() {
        DoubleMatrix::new(3, 2, vec![1f64; 100]);
    }

    #[test]
    fn test_new() {
        let matrix = DoubleMatrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 3);
        assert_eq!(matrix.data(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(matrix.shape(), (2, 3));
    }

    #[test]
    fn test_get_put_get() {
        let mut matrix = DoubleMatrix::ones(34, 56);
        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                assert_eq!(matrix.get(i, j), 1.0);
            }
        }

        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                matrix.put(i, j, (i + j) as f64);
            }
        }

        for i in 0..matrix.rows() {
            for j in 0..matrix.cols() {
                assert_eq!(matrix.get(i, j), (i + j) as f64);
            }
        }
    }

    #[test]
    fn test_identity() {
        let matrix = DoubleMatrix::identity(3, 3);
        assert_eq!(matrix.data(), &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0
        ]);

        let matrix = DoubleMatrix::identity(3, 4);
        assert_eq!(matrix.data(), &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 0.0
        ]);

        let matrix = DoubleMatrix::identity(4, 3);
        assert_eq!(matrix.data(), &[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0
        ]);
    }
}
