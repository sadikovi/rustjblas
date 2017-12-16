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

use std;
use nalgebra::{Dynamic, Matrix, MatrixVec};

// Dynamically sized and dynamically allocated float matrix
pub type DoubleMatrix = Matrix<f64, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>;

// Compute column sums
pub fn column_sums(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![0f64; matrix.ncols()];
    for col in 0..matrix.ncols() {
        vec[col] = matrix.column(col).iter().sum();
    }
    DoubleMatrix::from_column_slice(1, matrix.ncols(), &vec[..])
}

// Compute column mins
pub fn column_mins(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![std::f64::NAN; matrix.ncols()];
    for col in 0..matrix.ncols() {
        for &e in matrix.column(col).iter() {
            if vec[col].is_nan() || e < vec[col] {
                vec[col] = e;
            }
        }
    }
    DoubleMatrix::from_column_slice(1, matrix.ncols(), &vec[..])
}

// Compute column maxs
pub fn column_maxs(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![std::f64::NAN; matrix.ncols()];
    for col in 0..matrix.ncols() {
        for &e in matrix.column(col).iter() {
            if vec[col].is_nan() || e > vec[col] {
                vec[col] = e;
            }
        }
    }
    DoubleMatrix::from_column_slice(1, matrix.ncols(), &vec[..])
}

// Compute column means
pub fn column_means(matrix: &DoubleMatrix) -> DoubleMatrix {
    let rows = matrix.nrows() as f64; // number of elements for each column
    let sums = column_sums(matrix);
    sums.map(|value| value / rows)
}

// Compute row sums
pub fn row_sums(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![0f64; matrix.nrows()];
    for row in 0..matrix.nrows() {
        vec[row] = matrix.row(row).iter().sum();
    }
    DoubleMatrix::from_column_slice(matrix.nrows(), 1, &vec[..])
}

// Compute row mins
pub fn row_mins(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![std::f64::NAN; matrix.nrows()];
    for row in 0..matrix.nrows() {
        for &e in matrix.row(row).iter() {
            if vec[row].is_nan() || e < vec[row] {
                vec[row] = e;
            }
        }
    }
    DoubleMatrix::from_column_slice(matrix.nrows(), 1, &vec[..])
}

// Compute row maxs
pub fn row_maxs(matrix: &DoubleMatrix) -> DoubleMatrix {
    let mut vec = vec![std::f64::NAN; matrix.nrows()];
    for row in 0..matrix.nrows() {
        for &e in matrix.row(row).iter() {
            if vec[row].is_nan() || e > vec[row] {
                vec[row] = e;
            }
        }
    }
    DoubleMatrix::from_column_slice(matrix.nrows(), 1, &vec[..])
}

// Compute column means
pub fn row_means(matrix: &DoubleMatrix) -> DoubleMatrix {
    let cols = matrix.ncols() as f64; // number of elements for each row
    let sums = row_sums(matrix);
    sums.map(|value| value / cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix(a: &DoubleMatrix, b: &DoubleMatrix) {
        let epsilon: f64 = 1e-8;
        assert_eq!(a.shape(), b.shape(), "Shape mismatch: {:?} != {:?}", a, b);
        let vec1 = a.data.data();
        let vec2 = b.data.data();
        for i in 0..vec1.len() {
            assert!((vec1[i] - vec2[i]).abs() <= epsilon,
                "Element mismatch {} != {}; a: {:?}, b: {:?}", vec1[i], vec2[i], a, b);
        }
    }

    fn test_matrix_1() -> DoubleMatrix {
        DoubleMatrix::from_row_slice(3, 4, &[
            0.25, 0.16, 0.03, 0.23,
            0.42, 0.33, 0.52, 0.27,
            0.71, 0.94, 0.37, 0.58
        ])
    }

    #[test]
    fn test_column_sums() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(1, 4, &[1.38, 1.43, 0.92, 1.08]);
        assert_matrix(&column_sums(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        assert_matrix(&column_sums(&matrix), &matrix);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[6.0]);
        assert_matrix(&column_sums(&matrix), &exp);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        assert_matrix(&column_sums(&matrix), &matrix);
    }

    #[test]
    fn test_column_mins() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(1, 4, &[0.25, 0.16, 0.03, 0.23]);
        assert_matrix(&column_mins(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        assert_matrix(&column_mins(&matrix), &matrix);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[1.0]);
        assert_matrix(&column_mins(&matrix), &exp);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        assert_matrix(&column_mins(&matrix), &matrix);
    }

    #[test]
    fn test_column_maxs() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(1, 4, &[0.71, 0.94, 0.52, 0.58]);
        assert_matrix(&column_maxs(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        assert_matrix(&column_mins(&matrix), &matrix);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        assert_matrix(&column_maxs(&matrix), &exp);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        assert_matrix(&column_maxs(&matrix), &matrix);
    }

    #[test]
    fn test_column_means() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(1, 4, &[0.46, 0.47666667, 0.30666667, 0.36]);
        assert_matrix(&column_means(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        assert_matrix(&column_means(&matrix), &matrix);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[21.0, 12.0, 36.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[23.0]);
        assert_matrix(&column_means(&matrix), &exp);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        assert_matrix(&column_means(&matrix), &matrix);
    }

    #[test]
    fn test_row_sums() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(3, 1, &[0.67, 1.54, 2.6]);
        assert_matrix(&row_sums(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 4, &[0.25, 0.16, 0.03, 0.23]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[0.67]);
        assert_matrix(&row_sums(&matrix), &exp);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[0.1, 0.2, 0.3]);
        assert_matrix(&row_sums(&matrix), &matrix);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[0.43]);
        assert_matrix(&row_sums(&matrix), &matrix);
    }

    #[test]
    fn test_row_mins() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(3, 1, &[0.03, 0.27, 0.37]);
        assert_matrix(&row_mins(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[0.1, 0.2, 0.3]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[0.1]);
        assert_matrix(&row_mins(&matrix), &exp);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[0.1, 0.2, 0.3]);
        assert_matrix(&row_mins(&matrix), &matrix);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[0.43]);
        assert_matrix(&row_mins(&matrix), &matrix);
    }

    #[test]
    fn test_row_maxs() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(3, 1, &[0.25, 0.52, 0.94]);
        assert_matrix(&row_maxs(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[0.1, 0.2, 0.3]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[0.3]);
        assert_matrix(&row_maxs(&matrix), &exp);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[0.1, 0.2, 0.3]);
        assert_matrix(&row_maxs(&matrix), &matrix);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[0.43]);
        assert_matrix(&row_maxs(&matrix), &matrix);
    }

    #[test]
    fn test_row_means() {
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::from_row_slice(3, 1, &[0.1675, 0.385, 0.65]);
        assert_matrix(&row_means(&matrix), &exp);

        // row vector
        let matrix = DoubleMatrix::from_row_slice(1, 4, &[0.71, 0.94, 0.37, 0.58]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[0.65]);
        assert_matrix(&row_means(&matrix), &exp);

        // column vector
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[0.1, 0.2, 0.3]);
        assert_matrix(&row_means(&matrix), &matrix);

        // single element
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[0.43]);
        assert_matrix(&row_means(&matrix), &matrix);
    }
}
