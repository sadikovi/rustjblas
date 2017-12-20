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
use std::cmp;
use blas::dgemm;
use lapack::dgesdd;
use nalgebra::{Dynamic, Matrix, MatrixVec};
use nalgebra::storage::Storage;
use rand::{Rng, weak_rng};

// Dynamically sized and dynamically allocated float matrix
pub type DoubleMatrix = Matrix<f64, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>;

// Singular value decomposition struct
pub struct SVD {
    pub u: Option<DoubleMatrix>, // left singular vectors
    pub s: DoubleMatrix, // singular values as column vector
    pub v: Option<DoubleMatrix> // right singular vectors
}

// Create new DoubleMatrix from shape and dynamic data
#[inline]
fn new_double_matrix(nrows: usize, ncols: usize, data: Vec<f64>) -> DoubleMatrix {
    let rows = Dynamic::new(nrows);
    let cols = Dynamic::new(ncols);
    let mvec = MatrixVec::new(rows, cols, data);
    DoubleMatrix::from_data(mvec)
}

// Generate matrix of random values
// This method is faster than library method
pub fn new_random(nrows: usize, ncols: usize) -> DoubleMatrix {
    let mut rng = weak_rng();
    let data = rng.gen_iter::<f64>().take(nrows * ncols).collect::<Vec<f64>>();
    new_double_matrix(nrows, ncols, data)
}

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

// Matrix multiply c = a * b using blas
fn mmul_to(a: &DoubleMatrix, b: &DoubleMatrix, c: &mut DoubleMatrix) {
    let (arows, acols) = a.shape();
    let (brows, bcols) = b.shape();
    let (crows, ccols) = c.shape();

    assert_eq!(acols, brows, "input dimensions mismatch for multiplication.");
    assert_eq!((crows, ccols), (arows, bcols), "output dimensions mismatch for multiplication.");

    let m = arows as i32;
    let n = bcols as i32;
    let k = acols as i32;

    let alpha = 1f64;
    let beta = 0f64;

    unsafe {
        dgemm(
            'N' as u8, // transa: u8,
            'N' as u8, // transb: u8,
            m, // m: i32,
            n, // n: i32,
            k, // k: i32,
            alpha, // alpha: f64,
            a.data.data(), // a: &[f64],
            cmp::max(1, m), // lda: i32,
            b.data.data(), // b: &[f64],
            cmp::max(1, k), // ldb: i32,
            beta, // beta: f64,
            c.data.data_mut(), // c: &mut [f64],
            cmp::max(1, m) // ldc: i32
        );
    }
}

// Matrix multiply
pub fn mmul(a: &DoubleMatrix, b: &DoubleMatrix) -> DoubleMatrix {
    let mut res = unsafe {
        DoubleMatrix::new_uninitialized_generic(a.data.shape().0, b.data.shape().1)
    };
    mmul_to(a, b, &mut res);
    res
}

// Matrix multiply in-place
pub fn mmul_assign(a: &mut DoubleMatrix, b: &DoubleMatrix) {
    *a = mmul(&*a, b);
}

// Compute the singular value decomposition (SVD) of a real M-by-N matrix, also computing the left
// and right singular vectors, for which it uses a divide-and-conquer algorithm.
pub fn full_svd(matrix: &DoubleMatrix) -> SVD {
    let (rows, cols) = matrix.shape();

    // here we compute both left and right singular vectors and hard-code value of jobz
    // also need to copy content of a, since it can be modified, have we decided to change mode
    let mut a = matrix.data.data().clone();
    // singular values vector
    let srows = cmp::min(rows, cols);
    let scols = 1;
    let mut s = vec![0f64; srows * scols];
    // left singular vectors
    let urows = rows;
    let ucols = rows;
    let mut u = vec![0f64; urows * ucols];
    // right singular vectors
    let vtrows = cols;
    let vtcols = cols;
    let mut vt = vec![0f64; vtrows * vtcols];
    let mut iwork = vec![0i32; 8 * cmp::min(rows, cols)];
    let mut info = 0i32;

    // estimate size of lwork
    let lwork = -1;
    let mut work = vec![0f64; 1];
    unsafe {
        dgesdd(
            'A' as u8, // jobz: u8,
            rows as i32, // m: i32,
            cols as i32, // n: i32,
            &mut vec![], // a: &mut [f64],
            cmp::max(1, rows) as i32, // lda: i32,
            &mut vec![], // s: &mut [f64],
            &mut vec![], // u: &mut [f64],
            cmp::max(1, urows) as i32, // ldu: i32,
            &mut vec![], // vt: &mut [f64],
            cmp::max(1, vtrows) as i32, // ldvt: i32,
            &mut work, // work: &mut [f64],
            lwork, // lwork: i32,
            &mut vec![], // iwork: &mut [i32],
            &mut info // info: &mut i32
        );
    }

    assert!(info == 0, "Workspace query failed to execute with code {}", info);

    // additional workspace data structures after adjustment
    let lwork = work[0] as usize;
    let mut work = vec![0f64; lwork];

    unsafe {
        dgesdd(
            'A' as u8, // jobz: u8,
            rows as i32, // m: i32,
            cols as i32, // n: i32,
            &mut a, // a: &mut [f64],
            cmp::max(1, rows) as i32, // lda: i32,
            &mut s, // s: &mut [f64],
            &mut u, // u: &mut [f64],
            cmp::max(1, urows) as i32, // ldu: i32,
            &mut vt, // vt: &mut [f64],
            cmp::max(1, vtrows) as i32, // ldvt: i32,
            &mut work, // work: &mut [f64],
            lwork as i32, // lwork: i32,
            &mut iwork, // iwork: &mut [i32],
            &mut info // info: &mut i32
        );
    }

    // TODO: report that -i th element has illegal value
    assert!(info <= 0, "GESDD did not converge, {}", info);

    let u = new_double_matrix(urows, ucols, u);
    let s = new_double_matrix(srows, scols, s);
    // v is returned as vt, so we transpose it in-place
    let mut v = new_double_matrix(vtrows, vtcols, vt);
    v.transpose_mut();

    SVD { u: Some(u), s: s, v: Some(v) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix_eps(a: &DoubleMatrix, b: &DoubleMatrix, epsilon: f64) {
        assert_eq!(a.shape(), b.shape(), "Shape mismatch: {:?} != {:?}", a, b);
        let vec1 = a.data.data();
        let vec2 = b.data.data();
        for i in 0..vec1.len() {
            assert!((vec1[i] - vec2[i]).abs() <= epsilon,
                "Element mismatch {} != {}; a: {:?}, b: {:?}", vec1[i], vec2[i], a, b);
        }
    }

    fn assert_matrix(a: &DoubleMatrix, b: &DoubleMatrix) {
        assert_matrix_eps(a, b, 1e-8);
    }

    fn test_matrix_1() -> DoubleMatrix {
        DoubleMatrix::from_row_slice(3, 4, &[
            0.25, 0.16, 0.03, 0.23,
            0.42, 0.33, 0.52, 0.27,
            0.71, 0.94, 0.37, 0.58
        ])
    }

    fn test_matrix_2() -> DoubleMatrix {
        DoubleMatrix::from_row_slice(4, 4, &[
            1.0, 1.0, 0.0, 0.0,
            0.0, 2.0, 1.0, 0.0,
            0.0, 0.0, 3.0, 1.0,
            0.0, 0.0, 0.0, 4.0
        ])
    }

    fn test_matrix_3() -> DoubleMatrix {
        DoubleMatrix::from_row_slice(2, 4, &[
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0
        ])
    }

    fn test_matrix_4() -> DoubleMatrix {
        DoubleMatrix::from_row_slice(3, 2, &[
            1.0, 2.0,
            2.0, 4.0,
            3.0, 5.0
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

    #[test]
    fn test_mmul() {
        let a = DoubleMatrix::new_random(10, 8);
        let b = DoubleMatrix::new_random(8, 20);
        let res = mmul(&a, &b);
        let exp = &a * &b;
        assert_matrix(&res, &exp);
    }

    #[test]
    fn test_mmul_vector_1() {
        let a = DoubleMatrix::new_random(10, 1);
        let b = DoubleMatrix::new_random(1, 8);
        let res = mmul(&a, &b);
        let exp = &a * &b;
        assert_matrix(&res, &exp);
    }

    #[test]
    fn test_mmul_vector_2() {
        let a = DoubleMatrix::new_random(1, 10);
        let b = DoubleMatrix::new_random(10, 1);
        let res = mmul(&a, &b);
        let exp = &a * &b;
        assert_matrix(&res, &exp);
    }

    #[test]
    fn test_mmul_assign() {
        let a = DoubleMatrix::new_random(10, 8);
        let b = DoubleMatrix::new_random(8, 20);
        let mut res = a.clone();
        mmul_assign(&mut res, &b);
        let mut exp = a.clone();
        exp *= &b;
        assert_matrix(&res, &exp);
    }

    #[test]
    fn test_full_svd_test_matrix_2() {
        let a = test_matrix_2();
        let svd = full_svd(&a);

        let u_exp = DoubleMatrix::from_row_slice(4, 4, &[
            0.013543, -0.135435, 0.542638, 0.828866,
            0.109341, -0.518419, 0.667345, -0.523390,
            0.470163, -0.714229, -0.481962, 0.191143,
            0.875676, 0.450306, 0.167053, -0.050094
        ]);
        let s_exp = DoubleMatrix::from_column_slice(4, 1, &[
            4.260007, 3.107349, 2.111785, 0.858542
        ]);
        let v_exp = DoubleMatrix::from_row_slice(4, 4, &[
            0.003179, -0.043585, 0.256957, 0.965434,
            0.054513, -0.377258, 0.888977, -0.253819,
            0.356767, -0.856391, -0.368665, 0.058285,
            0.932596, 0.349815, 0.088195, -0.010752
        ]);

        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_full_svd_test_matrix_3() {
        let a = test_matrix_3();
        let svd = full_svd(&a);

        let u_exp = DoubleMatrix::from_row_slice(2, 2, &[
            -0.376168, -0.926551,
            -0.926551, 0.376168
        ]);
        let s_exp = DoubleMatrix::from_column_slice(2, 1, &[
            14.227407, 1.257330
        ]);
        let v_exp = DoubleMatrix::from_row_slice(4, 4, &[
            -0.352062, 0.758981, -0.400087, -0.374072,
            -0.443626, 0.321242, 0.254633, 0.796971,
            -0.535190, -0.116498, 0.690996, -0.471724,
            -0.626754, -0.554238, -0.545542, 0.048826
        ]);

        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_full_svd_test_matrix_4() {
        let a = test_matrix_4();
        let svd = full_svd(&a);

        let u_exp = DoubleMatrix::from_row_slice(3, 3, &[
            -0.291036, -0.339556, -0.894427,
            -0.582071, -0.679112, 0.447214,
            -0.759270, 0.650776, -0.000000
        ]);
        let s_exp = DoubleMatrix::from_column_slice(2, 1, &[
            7.675619, 0.291321
        ]);
        let v_exp = DoubleMatrix::from_row_slice(2, 2, &[
            -0.486344, 0.873768,
            -0.873768, -0.486344
        ]);

        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }
}
