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
use std::f64::{EPSILON, NAN};
use std::fmt::{Display, Error, Formatter};
use blas::{dasum, daxpy, dcopy, dgemm, dnrm2, dscal};
use lapack::{dgesdd, dgesvdx};
use rand::{Rng, weak_rng};
use lanczos::dlansvd_irl;

// Macro to assert matrices shapes
macro_rules! assert_shape {
    ($x:expr, $y:expr) => (
        assert_eq!($x, $y, "Shape mismatch: {:?} != {:?}.", $x, $y);
    )
}

// Macro to generate vectorized elementwise matrix operations (experimental)
macro_rules! vectorized_op {
    ($fn_matrix_mut:ident, $fn_matrix:ident, $op:tt) => (
        #[inline]
        pub fn $fn_matrix_mut(&mut self, other: &DoubleMatrix) {
            assert_shape!(self.shape(), other.shape());
            // load default non-vectorized implementation when instructions are not available
            let mut i = 0;
            let len = self.data.len();
            while i < len & !3 {
                self.data[i] = self.data[i] $op other.data[i];
                self.data[i+1] = self.data[i+1] $op other.data[i+1];
                self.data[i+2] = self.data[i+2] $op other.data[i+2];
                self.data[i+3] = self.data[i+3] $op other.data[i+3];
                i += 4;
            }
            while i < len {
                self.data[i] = self.data[i] $op other.data[i];
                i += 1;
            }
        }

        #[inline]
        pub fn $fn_matrix(&self, other: &DoubleMatrix) -> Self {
            let mut clone = self.clone();
            clone.$fn_matrix_mut(other);
            clone
        }
    )
}

macro_rules! dgesdd_op {
    ($jobz:expr, $a:ident, $rows:ident, $cols:ident, $u:ident, $urows:ident, $s:ident, $vt:ident,
        $vtrows:ident) => {{

        let mut iwork = vec![0i32; 8 * cmp::min($rows, $cols)];
        let mut info = 0i32;

        // estimate size of lwork
        let lwork = -1;
        let mut work = vec![0f64; 1];

        unsafe {
            dgesdd(
                $jobz as u8, // jobz: u8,
                $rows as i32, // m: i32,
                $cols as i32, // n: i32,
                &mut vec![], // a: &mut [f64],
                cmp::max(1, $rows) as i32, // lda: i32,
                &mut vec![], // s: &mut [f64],
                &mut vec![], // u: &mut [f64],
                cmp::max(1, $urows) as i32, // ldu: i32,
                &mut vec![], // vt: &mut [f64],
                cmp::max(1, $vtrows) as i32, // ldvt: i32,
                &mut work, // work: &mut [f64],
                lwork, // lwork: i32,
                &mut vec![], // iwork: &mut [i32],
                &mut info // info: &mut i32
            );
        }

        assert!(info == 0, "Workspace query failed to execute with code {}.", info);

        // additional workspace data structures after adjustment
        let lwork = work[0] as usize;
        let mut work = vec![0f64; lwork];

        unsafe {
            dgesdd(
                $jobz as u8, // jobz: u8,
                $rows as i32, // m: i32,
                $cols as i32, // n: i32,
                &mut $a, // a: &mut [f64],
                cmp::max(1, $rows) as i32, // lda: i32,
                &mut $s, // s: &mut [f64],
                &mut $u, // u: &mut [f64],
                cmp::max(1, $urows) as i32, // ldu: i32,
                &mut $vt, // vt: &mut [f64],
                cmp::max(1, $vtrows) as i32, // ldvt: i32,
                &mut work, // work: &mut [f64],
                lwork as i32, // lwork: i32,
                &mut iwork, // iwork: &mut [i32],
                &mut info // info: &mut i32
            );
        }

        // this is strict check; when info is negative, then ith parameter has illegal value
        assert!(info == 0, "GESDD did not converge, {}.", info);
    }}
}

macro_rules! dcopy {
    ($dx:ident) => {{
        let mut dy = vec![0f64; $dx.len()];
        unsafe { dcopy($dx.len() as i32, $dx, 1i32, &mut dy, 1i32); }
        dy
    }};
    ($dx:expr) => {{
        let dx = $dx;
        let mut dy = vec![0f64; dx.len()];
        unsafe { dcopy(dx.len() as i32, dx, 1i32, &mut dy, 1i32); }
        dy
    }};
    ($dx:ident, $dy:ident) => {{
        // caller should ensure that dx.len == dy.len
        unsafe { dcopy($dx.len() as i32, $dx, 1i32, &mut $dy, 1i32); }
    }}
}

// Strict representation of the double matrix with as little overhead as possible.
// This allows us to resolve library conflicts and implement transformations efficiently.
#[derive(Debug)]
pub struct DoubleMatrix {
    rows: usize, // number of rows in this matrix
    cols: usize, // number of columns in this matrix
    data: Vec<f64> // data in column major order
}

// Singular value decomposition struct
#[derive(Debug)]
pub struct SVD {
    pub u: Option<DoubleMatrix>, // left singular vectors
    pub s: DoubleMatrix, // singular values as column vector
    pub v: Option<DoubleMatrix> // right singular vectors
}

impl DoubleMatrix {
    // Create new matrix from data in column-major order
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len(),
            "Dimensions mismatch: {} * {} != {}.", rows, cols, data.len());
        // use shrink_to_fit
        DoubleMatrix { rows: rows, cols: cols, data: data }
    }

    // Create new matrix from data in row-major order, used only for testing
    // TODO: when making public, consider optimising this method
    pub fn from_row_slice(rows: usize, cols: usize, data: &[f64]) -> Self {
        assert_eq!(rows * cols, data.len(),
            "Dimensions mismatch: {} * {} != {}.", rows, cols, data.len());
        let mut matrix = Self::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                matrix.put(i, j, data[cols * i + j]);
            }
        }
        matrix
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

    // Elementwise matrix-scalar operations

    #[inline]
    pub fn add_scalar_mut(&mut self, value: f64) {
        let dy = self.data_mut();
        unsafe { daxpy(dy.len() as i32, 1f64, &[value], 0i32, dy, 1i32); }
    }

    #[inline]
    pub fn sub_scalar_mut(&mut self, value: f64) {
        self.add_scalar_mut(-value);
    }

    #[inline]
    pub fn mul_scalar_mut(&mut self, value: f64) {
        let x = self.data_mut();
        unsafe { dscal(x.len() as i32, value, x, 1i32); }
    }

    #[inline]
    pub fn div_scalar_mut(&mut self, value: f64) {
        self.mul_scalar_mut(1f64 / value);
    }

    #[inline]
    pub fn add_scalar(&self, value: f64) -> DoubleMatrix {
        let mut clone = self.clone();
        clone.add_scalar_mut(value);
        clone
    }

    #[inline]
    pub fn sub_scalar(&self, value: f64) -> DoubleMatrix {
        self.add_scalar(-value)
    }

    #[inline]
    pub fn mul_scalar(&self, value: f64) -> DoubleMatrix {
        let mut clone = self.clone();
        clone.mul_scalar_mut(value);
        clone
    }

    #[inline]
    pub fn div_scalar(&self, value: f64) -> DoubleMatrix {
        self.mul_scalar(1f64 / value)
    }

    // Elementwise matrix-matrix operations

    #[inline]
    pub fn add_matrix_mut(&mut self, other: &DoubleMatrix) {
        assert_shape!(self.shape(), other.shape());
        let y = self.data_mut();
        let x = other.data();
        unsafe { daxpy(y.len() as i32, 1f64, x, 1i32, y, 1i32); }
    }

    #[inline]
    pub fn add_matrix(&self, other: &DoubleMatrix) -> DoubleMatrix {
        let mut clone = self.clone();
        clone.add_matrix_mut(other);
        clone
    }

    #[inline]
    pub fn sub_matrix_mut(&mut self, other: &DoubleMatrix) {
        assert_shape!(self.shape(), other.shape());
        let y = self.data_mut();
        let x = other.data();
        unsafe { daxpy(y.len() as i32, -1f64, x, 1i32, y, 1i32); }
    }

    #[inline]
    pub fn sub_matrix(&self, other: &DoubleMatrix) -> DoubleMatrix {
        let mut clone = self.clone();
        clone.sub_matrix_mut(other);
        clone
    }

    // TODO: convert these methods using blas, e.g. dgbmv
    vectorized_op!(mul_matrix_mut, mul_matrix, *);
    vectorized_op!(div_matrix_mut, div_matrix, /);

    // == Matrix operations ==

    // Matrix multiply c = a * b using blas
    fn mmul_to(a: &DoubleMatrix, b: &DoubleMatrix, c: &mut DoubleMatrix) {
        let (arows, acols) = a.shape();
        let (brows, bcols) = b.shape();
        let (crows, ccols) = c.shape();

        assert_eq!(acols, brows, "Input dimensions mismatch for multiplication.");
        assert_eq!((crows, ccols), (arows, bcols), "Output dimensions mismatch for multiplication.");

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
                a.data(), // a: &[f64],
                cmp::max(1, m), // lda: i32,
                b.data(), // b: &[f64],
                cmp::max(1, k), // ldb: i32,
                beta, // beta: f64,
                c.data_mut(), // c: &mut [f64],
                cmp::max(1, m) // ldc: i32
            );
        }
    }

    // Matrix multiply
    pub fn mmul(&self, b: &DoubleMatrix) -> DoubleMatrix {
        let mut res = DoubleMatrix::zeros(self.rows(), b.cols());
        Self::mmul_to(&self, b, &mut res);
        res
    }

    // Matrix multiply in-place
    pub fn mmul_assign(&mut self, b: &DoubleMatrix) {
        *self = self.mmul(b);
    }

    // Compute column sums
    pub fn column_sums(&self) -> DoubleMatrix {
        let mut vec = vec![0f64; self.cols()];
        for c in 0..self.cols() {
            for r in 0..self.rows() {
                vec[c] += self.get(r, c);
            }
        }
        DoubleMatrix::new(1, self.cols(), vec)
    }

    // Compute column mins
    pub fn column_mins(&self) -> DoubleMatrix {
        let mut vec = vec![NAN; self.cols()];
        for c in 0..self.cols() {
            for r in 0..self.rows() {
                let e = self.get(r, c);
                if vec[c].is_nan() || e < vec[c] {
                    vec[c] = e;
                }
            }
        }
        DoubleMatrix::new(1, self.cols(), vec)
    }

    // Compute column maxs
    pub fn column_maxs(&self) -> DoubleMatrix {
        let mut vec = vec![NAN; self.cols()];
        for c in 0..self.cols() {
            for r in 0..self.rows() {
                let e = self.get(r, c);
                if vec[c].is_nan() || e > vec[c] {
                    vec[c] = e;
                }
            }
        }
        DoubleMatrix::new(1, self.cols(), vec)
    }

    // Compute column means
    pub fn column_means(&self) -> DoubleMatrix {
        let rows = self.rows() as f64; // number of elements for each column
        let mut sums = self.column_sums();
        sums.div_scalar_mut(rows);
        sums
    }

    // Compute row sums
    pub fn row_sums(&self) -> DoubleMatrix {
        let mut vec = vec![0f64; self.rows()];
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                vec[r] += self.get(r, c);
            }
        }
        DoubleMatrix::new(self.rows(), 1, vec)
    }

    // Compute row mins
    pub fn row_mins(&self) -> DoubleMatrix {
        let mut vec = vec![NAN; self.rows()];
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let e = self.get(r, c);
                if vec[r].is_nan() || e < vec[r] {
                    vec[r] = e;
                }
            }
        }
        DoubleMatrix::new(self.rows(), 1, vec)
    }

    // Compute row maxs
    pub fn row_maxs(&self) -> DoubleMatrix {
        let mut vec = vec![NAN; self.rows()];
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                let e = self.get(r, c);
                if vec[r].is_nan() || e > vec[r] {
                    vec[r] = e;
                }
            }
        }
        DoubleMatrix::new(self.rows(), 1, vec)
    }

    // Compute column means
    pub fn row_means(&self) -> DoubleMatrix {
        let cols = self.cols() as f64; // number of elements for each row
        let mut sums = self.row_sums();
        sums.div_scalar_mut(cols);
        sums
    }

    // Put (update) column vector in this matrix for a column index
    pub fn put_column(&mut self, col: usize, vector: &DoubleMatrix) {
        assert_eq!(vector.shape(), (self.rows(), 1),
            "Invalid shape for column vector: {:?}.", vector.shape());
        assert!(col < self.cols(), "Invalid column index {} ({} columns).", col, self.cols());
        let start = self.m2v(0, col);
        let end = self.m2v(self.rows() - 1, col);
        let dx = vector.data();
        let mut dy = &mut self.data[start..end];
        dcopy![dx, dy];
    }

    // Get column for index col from this matrix as column vector
    pub fn get_column(&self, col: usize) -> DoubleMatrix {
        assert!(col < self.cols(), "Invalid column index {} ({} columns).", col, self.cols());
        let mut vector = DoubleMatrix::zeros(self.rows(), 1);
        {
            let start = self.m2v(0, col);
            let end = self.m2v(self.rows() - 1, col);
            let dx = &self.data[start..end+1];
            let mut dy = &mut vector.data;
            dcopy![dx, dy];
        }
        vector
    }

    // Put (update) row vector in this matrix for a row index
    pub fn put_row(&mut self, row: usize, vector: &DoubleMatrix) {
        assert_eq!(vector.shape(), (1, self.cols()),
            "Invalid shape for row vector: {:?}.", vector.shape());
        assert!(row < self.rows(), "Invalid row index {} ({} rows).", row, self.rows());
        for col in 0..self.cols() {
            self.put(row, col, vector.get(0, col));
        }
    }

    // Get row for index row from this matrix as row vector
    pub fn get_row(&self, row: usize) -> DoubleMatrix {
        assert!(row < self.rows(), "Invalid row index {} ({} rows).", row, self.rows());
        let mut vector = vec![0f64; self.cols()];
        for col in 0..self.cols() {
            vector[col] = self.get(row, col);
        }
        DoubleMatrix::new(1, self.cols(), vector)
    }

    // Find min element in matrix
    pub fn min(&self) -> f64 {
        let mut min = NAN;
        for &elem in self.data() {
            if min.is_nan() || elem < min {
                min = elem;
            }
        }
        min
    }

    // Find max element in matrix
    pub fn max(&self) -> f64 {
        let mut max = NAN;
        for &elem in self.data() {
            if max.is_nan() || elem > max {
                max = elem;
            }
        }
        max
    }

    // Compute sum of elements in matrix
    pub fn sum(&self) -> f64 {
        let mut sum = 0f64;
        for &elem in self.data() {
            sum += elem;
        }
        sum
    }

    // The 1-norm of the matrix as vector (sum of absolute values of elements).
    pub fn norm1(&self) -> f64 {
        let a = self.data();
        unsafe { dasum(a.len() as i32, a, 1i32) }
    }

    // The Euclidean norm of the matrix as vector, also the Frobenius norm of the matrix.
    pub fn norm2(&self) -> f64 {
        let a = self.data();
        unsafe { dnrm2(a.len() as i32, a, 1i32) }
    }

    // Return transposed matrix
    #[inline]
    pub fn transpose(&self) -> DoubleMatrix {
        let mut tarr = vec![0f64; self.rows * self.cols];
        let arr = self.data();
        for i in 0..arr.len() {
            let row = i % self.rows;
            let col = i / self.rows;
            let ti = col + row * self.cols;
            tarr[ti] = arr[i];
        }
        DoubleMatrix::new(self.cols(), self.rows(), tarr)
    }

    // Transpose matrix in place
    pub fn transpose_mut(&mut self) {
        assert_eq!(self.rows(), self.cols(), "Unable to transpose a non-square matrix in-place.");
        let dim = self.rows();
        for i in 1 .. dim {
            for j in 0 .. i {
                let a = self.m2v(i, j);
                let b = self.m2v(j, i);
                self.data.swap(a, b);
            }
        }
    }

    // Return diagonal as column vector
    pub fn diag(&self) -> DoubleMatrix {
        assert_eq!(self.rows(), self.cols(), "Unable to get the diagonal of a non-square matrix.");
        let mut diag = DoubleMatrix::zeros(self.rows(), 1);
        for i in 0..self.rows() {
            diag.put(i, 0, self.get(i, i));
        }
        diag
    }

    // Return matrix of absolute values
    pub fn abs(&self) -> DoubleMatrix {
        let mut vec = vec![0f64; self.rows() * self.cols()];
        for i in 0..vec.len() {
            vec[i] = self.data[i].abs();
        }
        DoubleMatrix::new(self.rows(), self.cols(), vec)
    }

    // Compute the singular value decomposition (SVD) of a real M-by-N matrix, also computing the
    // left and right singular vectors, for which it uses a divide-and-conquer algorithm.
    pub fn full_svd(&self) -> SVD {
        let jobz = 'A';
        let (rows, cols) = self.shape();
        // need to copy content of a, since it can be modified, have we decided to change mode
        let mut a = dcopy![self.data()];
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

        dgesdd_op!(jobz, a, rows, cols, u, urows, s, vt, vtrows);

        let u = DoubleMatrix::new(urows, ucols, u);
        let s = DoubleMatrix::new(srows, scols, s);
        // v is returned as square vt, so we transpose it in-place
        let mut v = DoubleMatrix::new(vtrows, vtcols, vt);
        v.transpose_mut();

        SVD { u: Some(u), s: s, v: Some(v) }
    }

    // Compute all (up to epsilon) singular values for this matrix
    pub fn singular_values(&self) -> DoubleMatrix {
        let jobz = 'N';
        let (rows, cols) = self.shape();
        // need to copy content of a, since it can be modified, have we decided to change mode
        let mut a = dcopy![self.data()];
        // singular values vector
        let srows = cmp::min(rows, cols);
        let scols = 1;
        let mut s = vec![0f64; srows * scols];
        // do not compute left singular vectors
        let urows = rows;
        let mut u = vec![];
        // do not compute right singular vectors
        let vtrows = cols;
        let mut vt = vec![];

        dgesdd_op!(jobz, a, rows, cols, u, urows, s, vt, vtrows);

        DoubleMatrix::new(srows, scols, s)
    }

    // Experimental svd for top k singular values.
    // Currenty computes both left and right singular vectors, this behaviour can be changed later,
    // to allow choice of left or right or none to speed up computation.
    //
    // Based on DGESVDX that uses an eigenvalue problem for obtaining the SVD, which
    // allows for the computation of a subset of singular values and vectors.
    // See Lapack/DBDSVDX for details.
    pub fn svd(&self, k: usize) -> SVD {
        let (rows, cols) = self.shape();
        assert!(k >= 1 && k <= cmp::min(rows, cols), "Invalid number of singular values: {}.", k);

        let mut a = dcopy![self.data()];
        // singular values vector
        let mut ns = vec![0i32; 1]; // vector to contain number of singluar values found
        let (srows, scols) = (cmp::min(rows, cols), 1);
        let mut s = vec![0f64; srows * scols];
        // left singular vectors
        let (urows, ucols) = (rows, k);
        let mut u = vec![0f64; urows * ucols];
        // right singular vectors
        let (vtrows, vtcols) = (k, cols);
        let mut vt = vec![0f64; vtrows * vtcols];
        let mut iwork = vec![0i32; 12 * cmp::min(rows, cols)];
        let mut info = 0i32;

        // estimate size of lwork
        let lwork = -1;
        let mut work = vec![0f64; 1];

        unsafe {
            dgesvdx(
                'V' as u8, // jobu: u8,
                'V' as u8, // jobvt: u8,
                'I' as u8, // range: u8,
                rows as i32, // m: i32,
                cols as i32, // n: i32,
                &mut vec![], // a: &mut [f64],
                cmp::max(1, rows) as i32, // lda: i32,
                0f64, // vl: f64,
                0f64, // vu: f64,
                1i32, // il: i32,
                k as i32, // iu: i32,
                &mut ns, // ns: &mut [i32],
                &mut vec![], // s: &mut [f64],
                &mut vec![], // u: &mut [f64],
                cmp::max(1, urows) as i32, // ldu: i32,
                &mut vec![], // vt: &mut [f64],
                cmp::max(1, vtrows) as i32, // ldvt: i32,
                &mut work, // work: &mut [f64],
                lwork, // lwork: i32,
                &mut vec![], // iwork: &mut [i32],
                &mut info, // info: &mut i32
            );
        }

        assert!(info == 0, "Workspace query failed to execute with code {}.", info);

        // additional workspace data structures after adjustment
        let lwork = work[0] as usize;
        let mut work = vec![0f64; cmp::max(1, lwork)];

        unsafe {
            dgesvdx(
                'V' as u8, // jobu: u8,
                'V' as u8, // jobvt: u8,
                'I' as u8, // range: u8,
                rows as i32, // m: i32,
                cols as i32, // n: i32,
                &mut a, // a: &mut [f64],
                cmp::max(1, rows) as i32, // lda: i32,
                0f64, // vl: f64,
                0f64, // vu: f64,
                1i32, // il: i32,
                k as i32, // iu: i32,
                &mut ns, // ns: &mut [i32],
                &mut s, // s: &mut [f64],
                &mut u, // u: &mut [f64],
                cmp::max(1, urows) as i32, // ldu: i32,
                &mut vt, // vt: &mut [f64],
                cmp::max(1, vtrows) as i32, // ldvt: i32,
                &mut work, // work: &mut [f64],
                lwork as i32, // lwork: i32,
                &mut iwork, // iwork: &mut [i32],
                &mut info, // info: &mut i32
            );
        }

        // this is strict check; when info is negative, then ith parameter has illegal value
        assert!(info == 0, "GESVDX did not converge, {}.", info);
        assert!(ns[0] == k as i32,
            "GESVDX: {} (ns[0]) != {} (k), truncation is not supported.", ns[0], k);

        let u = DoubleMatrix::new(urows, ucols, u);
        s.truncate(k); // do not call dcopy on s, it is normally small (< 100)
        let s = DoubleMatrix::new(k, 1, s);
        // v is returned as vt, transpose by copy, because it is not square
        let v = DoubleMatrix::new(vtrows, vtcols, vt).transpose();
        SVD { u: Some(u), s: s, v: Some(v) }
    }

    // Experimental svd for top k singular values.
    // Currently computes both left and right singular vectors, this behaviour can be changed to
    // allow faster computation. Also would be good to expose some options, like tolerance and max
    // number of iterations.
    //
    // Based on DLANSVD_IRL that computes the leading singular triplets of a large and sparse matrix
    // A by implicitly restarted Lanczos bidiagonalization with partial reorthogonalization.
    // Note that current method uses A as dense matrix, which is influenced by using dense_matmul,
    // based on BLAS dgemv.
    pub fn lansvd(&self, k: usize) -> SVD {
        let (rows, cols) = self.shape();
        let lanmax = cmp::min(rows, cols);
        assert!(k >= 1 && k <= lanmax, "Invalid number of singular values: {}.", k);

        // number of desired singular triplets.
        let neig = cmp::min(k, lanmax);
        let kmax = cmp::min(7 * neig, lanmax + 1);
        // dimension of Krylov subspace
        let dim = kmax;
        // number of shift per restart
        let p = dim - neig;
        // maximum number of restarts (it looks like dlansvd_irl forces this number of iterations)
        // current value works well enough
        let maxiter = 3i32; // original value was 10
        // desired relative accuracy of computed singular values
        let tolin = 1e-8;

        // left singular vectors
        let (urows, ucols) = (rows, kmax + 1);
        let mut u = vec![0f64; urows * ucols];

        let mut sigma = vec![0f64; kmax];
        let mut bnd = vec![0f64; kmax];

        // right singular vectors
        let (vrows, vcols) = (kmax, cols);
        let mut v = vec![0f64; vrows * vcols];

        // size of work
        let lwork = rows + cols + 14 * kmax + 8 * kmax * kmax + 32 * rows + 9;
        // work buffer
        let mut work = vec![0f64; lwork];

        // size of iwork
        let liwork = 8 * kmax;
        let mut iwork = vec![0i32; liwork];

        // doption
        let doption = vec![
            // level of orthogonality to maintain among Lanczos vectors
            EPSILON.sqrt(),
            // during reorthogonalization, all vectors with with components larger than this value
            // along the latest Lanczos vector c will be purged
            EPSILON.powf(3.0 / 4.0),
            // estimate of || A ||
            0f64,
            // smallest relgap between any shift the smallest requested Ritz value
            0.002
        ];

        let ioption = vec![
            // reorthogonalization is done using iterated modified Gram-Schmidt
            0i32,
            // extended local orthogonality is enforced among u_{k}, u_{k+1} and v_{k} and v_{k+1}
            // respectively
            1i32
        ];

        // status info
        let mut info = 0i32;

        // dparm is *const f64, we do not change original matrix
        let dparm = self.data();
        // array used for passing data to the APROD function, not used for dense matrices
        let iparm = vec![];

        unsafe {
            dlansvd_irl(
                'L' as u8,
                'Y' as u8,
                'Y' as u8,
                rows as i32,
                cols as i32,
                dim as i32,
                p as i32,
                neig as i32,
                maxiter,
                &mut u,
                cmp::max(1, urows) as i32,
                &mut sigma,
                &mut bnd,
                &mut v,
                cmp::max(1, vcols) as i32,
                tolin,
                &mut work,
                lwork as i32,
                &mut iwork,
                liwork as i32,
                &doption,
                &ioption,
                &mut info,
                dparm,
                &iparm
            );
        }

        if info == -1 {
            panic!("DLANSVD_IRL, K singular triplets did not converge within KMAX iterations.");
        } else if info != 0 {
            panic!("DLANSVD_IRL, an invariant subspace of dimension J was found, {}.", info);
        }
        // at this point info == 0, K singular triplets were computed succesfully

        // truncate u to (rows, neig)
        let (urows, ucols) = (rows, neig);
        u.truncate(urows * ucols);
        let u = DoubleMatrix::new(urows, ucols, u);

        // truncate sigma to neig
        sigma.truncate(neig);
        let s = DoubleMatrix::new(neig, 1, sigma);

        // truncate v to (neig, cols)
        // note that v comes already transposed (not as v^T), so we just swap rows and cols
        let (vrows, vcols) = (cols, neig);
        v.truncate(vrows * vcols);
        let v = DoubleMatrix::new(vrows, vcols, v);

        SVD { u: Some(u), s: s, v: Some(v) }
    }
}

impl Clone for DoubleMatrix {
    fn clone(&self) -> Self {
        DoubleMatrix::new(self.rows, self.cols, dcopy![self.data()])
    }
}

impl Display for DoubleMatrix {
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

        let mut max_length_with_space = 0;
        let mut col_lengths = vec![0; cols];

        for j in 0..cols {
            for i in 0..rows {
                let idx = self.m2v(i, j);
                col_lengths[j] = cmp::max(col_lengths[j], val_width(self.data[idx], f));
            }
            max_length_with_space += col_lengths[j] + 1;
        }

        writeln!(f, "")?;
        writeln!(f, "  . {:>width$} .", "", width = max_length_with_space - 1)?;

        for i in 0..rows {
            write!(f, "  |")?;
            for j in 0..cols {
                let idx = self.m2v(i, j);
                let pad = col_lengths[j] - val_width(self.data[idx], f);
                write!(f, " {:>thepad$}", "", thepad = pad)?;
                match f.precision() {
                    Some(precision) => write!(f, "{:.1$}", self.data[idx], precision)?,
                    None => write!(f, "{}", self.data[idx])?
                }
            }
            writeln!(f, " |")?;
        }

        writeln!(f, "  . {:>width$} .", "", width = max_length_with_space - 1)?;
        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matrix_eps(a: &DoubleMatrix, b: &DoubleMatrix, epsilon: f64) {
        assert_eq!(a.shape(), b.shape(), "Shape mismatch: {:?} != {:?}", a, b);
        let vec1 = a.data();
        let vec2 = b.data();
        for i in 0..vec1.len() {
            assert!((vec1[i] - vec2[i]).abs() <= epsilon,
                "Element mismatch {} != {}; a: {:?}, b: {:?}", vec1[i], vec2[i], a, b);
        }
    }

    fn assert_matrix(a: &DoubleMatrix, b: &DoubleMatrix) {
        assert_matrix_eps(a, b, 1e-8);
    }

    fn assert_eq_f64_eps(a: f64, b: f64, eps: f64) {
        assert!((a - b).abs() <= eps, "Element mismatch {} != {}", a, b);
    }

    fn assert_eq_f64(a: f64, b: f64) {
        assert_eq_f64_eps(a, b, 1e-8);
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

    #[test]
    fn test_elementwise_add_scalar() {
        let matrix = DoubleMatrix::zeros(4, 5);
        let res = matrix.add_scalar(1f64);
        assert_matrix(&res, &DoubleMatrix::ones(4, 5));
    }

    #[test]
    fn test_elementwise_add_matrix() {
        let matrix = DoubleMatrix::zeros(4, 5);
        let res = matrix.add_matrix(&DoubleMatrix::ones(4, 5));
        assert_matrix(&res, &DoubleMatrix::ones(4, 5));
    }

    #[test]
    #[should_panic(expected = "Shape mismatch: (3, 4) != (2, 3).")]
    fn test_elementwise_add_matrix_panic() {
        DoubleMatrix::new_random(3, 4).add_matrix(&DoubleMatrix::new_random(2, 3));
    }

    #[test]
    fn test_elementwise_sub_scalar() {
        let matrix = DoubleMatrix::ones(4, 5);
        let res = matrix.sub_scalar(1f64);
        assert_matrix(&res, &DoubleMatrix::zeros(4, 5));
    }

    #[test]
    fn test_elementwise_sub_matrix() {
        let matrix = DoubleMatrix::ones(4, 5);
        let res = matrix.sub_matrix(&DoubleMatrix::ones(4, 5));
        assert_matrix(&res, &DoubleMatrix::zeros(4, 5));
    }

    #[test]
    #[should_panic(expected = "Shape mismatch: (3, 4) != (2, 3).")]
    fn test_elementwise_sub_matrix_panic() {
        DoubleMatrix::new_random(3, 4).sub_matrix(&DoubleMatrix::new_random(2, 3));
    }

    #[test]
    fn test_elementwise_mul_scalar() {
        let matrix = DoubleMatrix::ones(4, 5);
        let res = matrix.mul_scalar(3f64);
        let exp = matrix.add_scalar(1f64).add_scalar(1f64);
        assert_matrix(&res, &exp);
    }

    #[test]
    fn test_elementwise_mul_matrix() {
        let matrix = DoubleMatrix::ones(4, 5);
        let res = matrix.mul_matrix(&DoubleMatrix::identity(4, 5));
        assert_matrix(&res, &DoubleMatrix::identity(4, 5));
    }

    #[test]
    #[should_panic(expected = "Shape mismatch: (3, 4) != (2, 3).")]
    fn test_elementwise_mul_matrix_panic() {
        DoubleMatrix::new_random(3, 4).mul_matrix(&DoubleMatrix::new_random(2, 3));
    }

    #[test]
    fn test_elementwise_div_scalar() {
        let matrix = DoubleMatrix::ones(4, 5);
        let res = matrix.div_scalar(0.5);
        assert_matrix(&res, &matrix.mul_scalar(2f64));
    }

    #[test]
    fn test_elementwise_div_matrix() {
        let matrix = DoubleMatrix::new_random(4, 5);
        let res = matrix.div_matrix(&matrix);
        assert_matrix(&res, &DoubleMatrix::ones(4, 5));
    }

    #[test]
    #[should_panic(expected = "Shape mismatch: (3, 4) != (2, 3).")]
    fn test_elementwise_div_matrix_panic() {
        DoubleMatrix::new_random(3, 4).div_matrix(&DoubleMatrix::new_random(2, 3));
    }

    #[test]
    fn test_column_sums() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(1, 4, vec![1.38, 1.43, 0.92, 1.08]);
        assert_matrix(&matrix.column_sums(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![1.0, 2.0, 3.0]);
        assert_matrix(&matrix.column_sums(), &matrix);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::new(1, 1, vec![6.0]);
        assert_matrix(&matrix.column_sums(), &exp);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![3.0]);
        assert_matrix(&matrix.column_sums(), &matrix);
    }

    #[test]
    fn test_column_mins() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(1, 4, vec![0.25, 0.16, 0.03, 0.23]);
        assert_matrix(&matrix.column_mins(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![1.0, 2.0, 3.0]);
        assert_matrix(&matrix.column_mins(), &matrix);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::new(1, 1, vec![1.0]);
        assert_matrix(&matrix.column_mins(), &exp);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![3.0]);
        assert_matrix(&matrix.column_mins(), &matrix);
    }

    #[test]
    fn test_column_maxs() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(1, 4, vec![0.71, 0.94, 0.52, 0.58]);
        assert_matrix(&matrix.column_maxs(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![1.0, 2.0, 3.0]);
        assert_matrix(&matrix.column_mins(), &matrix);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::new(1, 1, vec![3.0]);
        assert_matrix(&matrix.column_maxs(), &exp);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![3.0]);
        assert_matrix(&matrix.column_maxs(), &matrix);
    }

    #[test]
    fn test_column_means() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(1, 4, vec![0.46, 0.47666667, 0.30666667, 0.36]);
        assert_matrix(&matrix.column_means(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![1.0, 2.0, 3.0]);
        assert_matrix(&matrix.column_means(), &matrix);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![21.0, 12.0, 36.0]);
        let exp = DoubleMatrix::new(1, 1, vec![23.0]);
        assert_matrix(&matrix.column_means(), &exp);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![3.0]);
        assert_matrix(&matrix.column_means(), &matrix);
    }

    #[test]
    fn test_row_sums() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(3, 1, vec![0.67, 1.54, 2.6]);
        assert_matrix(&matrix.row_sums(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 4, vec![0.25, 0.16, 0.03, 0.23]);
        let exp = DoubleMatrix::new(1, 1, vec![0.67]);
        assert_matrix(&matrix.row_sums(), &exp);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![0.1, 0.2, 0.3]);
        assert_matrix(&matrix.row_sums(), &matrix);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![0.43]);
        assert_matrix(&matrix.row_sums(), &matrix);
    }

    #[test]
    fn test_row_mins() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(3, 1, vec![0.03, 0.27, 0.37]);
        assert_matrix(&matrix.row_mins(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![0.1, 0.2, 0.3]);
        let exp = DoubleMatrix::new(1, 1, vec![0.1]);
        assert_matrix(&matrix.row_mins(), &exp);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![0.1, 0.2, 0.3]);
        assert_matrix(&matrix.row_mins(), &matrix);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![0.43]);
        assert_matrix(&matrix.row_mins(), &matrix);
    }

    #[test]
    fn test_row_maxs() {
        // full matrix
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(3, 1, vec![0.25, 0.52, 0.94]);
        assert_matrix(&matrix.row_maxs(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 3, vec![0.1, 0.2, 0.3]);
        let exp = DoubleMatrix::new(1, 1, vec![0.3]);
        assert_matrix(&matrix.row_maxs(), &exp);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![0.1, 0.2, 0.3]);
        assert_matrix(&matrix.row_maxs(), &matrix);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![0.43]);
        assert_matrix(&matrix.row_maxs(), &matrix);
    }

    #[test]
    fn test_row_means() {
        let matrix = test_matrix_1();
        let exp = DoubleMatrix::new(3, 1, vec![0.1675, 0.385, 0.65]);
        assert_matrix(&matrix.row_means(), &exp);

        // row vector
        let matrix = DoubleMatrix::new(1, 4, vec![0.71, 0.94, 0.37, 0.58]);
        let exp = DoubleMatrix::new(1, 1, vec![0.65]);
        assert_matrix(&matrix.row_means(), &exp);

        // column vector
        let matrix = DoubleMatrix::new(3, 1, vec![0.1, 0.2, 0.3]);
        assert_matrix(&matrix.row_means(), &matrix);

        // single element
        let matrix = DoubleMatrix::new(1, 1, vec![0.43]);
        assert_matrix(&matrix.row_means(), &matrix);
    }

    #[test]
    #[should_panic(expected = "Invalid shape for column vector: (4, 5).")]
    fn test_put_get_column_invalid_shape() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        matrix.put_column(1, &DoubleMatrix::ones(rows + 1, cols + 1));
    }

    #[test]
    #[should_panic(expected = "Invalid column index 4 (4 columns).")]
    fn test_put_get_column_invalid_index_1() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        matrix.put_column(cols, &DoubleMatrix::ones(rows, 1));
    }

    #[test]
    #[should_panic(expected = "Invalid column index 4 (4 columns).")]
    fn test_put_get_column_invalid_index_2() {
        let matrix = test_matrix_1();
        matrix.get_column(matrix.cols());
    }

    #[test]
    fn test_put_get_column() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        let vector = DoubleMatrix::ones(rows, 1);
        matrix.put_column(1, &vector);
        assert_matrix(&matrix.get_column(1), &vector);
        for col in 0..cols {
            matrix.put_column(col, &vector);
        }
        assert_matrix(&matrix, &DoubleMatrix::ones(rows, cols));
    }

    #[test]
    #[should_panic(expected = "Invalid shape for row vector: (4, 5).")]
    fn test_put_get_row_invalid_shape() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        matrix.put_row(1, &DoubleMatrix::ones(rows + 1, cols + 1));
    }

    #[test]
    #[should_panic(expected = "Invalid row index 3 (3 rows).")]
    fn test_put_get_row_invalid_index_1() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        matrix.put_row(rows, &DoubleMatrix::ones(1, cols));
    }

    #[test]
    #[should_panic(expected = "Invalid row index 3 (3 rows).")]
    fn test_put_get_row_invalid_index_2() {
        let matrix = test_matrix_1();
        matrix.get_row(matrix.rows());
    }

    #[test]
    fn test_put_get_row() {
        let mut matrix = test_matrix_1();
        let (rows, cols) = matrix.shape();
        let vector = DoubleMatrix::ones(1, cols);
        matrix.put_row(1, &vector);
        assert_matrix(&matrix.get_row(1), &vector);
        for row in 0..rows {
            matrix.put_row(row, &vector);
        }
        assert_matrix(&matrix, &DoubleMatrix::ones(rows, cols));
    }

    #[test]
    fn test_matrix_min() {
        assert_eq!(test_matrix_1().min(), 0.03);
        assert_eq!(test_matrix_2().min(), 0.0);
        assert_eq!(test_matrix_3().min(), 1.0);
        assert_eq!(test_matrix_4().min(), 1.0);
    }

    #[test]
    fn test_matrix_max() {
        assert_eq!(test_matrix_1().max(), 0.94);
        assert_eq!(test_matrix_2().max(), 4.0);
        assert_eq!(test_matrix_3().max(), 8.0);
        assert_eq!(test_matrix_4().max(), 5.0);
    }

    #[test]
    fn test_matrix_sum() {
        assert_eq!(test_matrix_1().sum(), 4.81);
        assert_eq!(test_matrix_2().sum(), 13.0);
        assert_eq!(test_matrix_3().sum(), 36.0);
        assert_eq!(test_matrix_4().sum(), 17.0);
    }

    #[test]
    fn test_norm1() {
        assert_eq_f64(test_matrix_1().norm1(), 4.81);
        assert_eq_f64(test_matrix_2().norm1(), 13.0);
        assert_eq_f64(test_matrix_3().norm1(), 36.0);
        assert_eq_f64(test_matrix_4().norm1(), 17.0);
    }

    #[test]
    fn test_norm2() {
        assert_eq_f64(test_matrix_1().norm2(), 1.622189877911954);
        assert_eq_f64(test_matrix_2().norm2(), 5.744562646538029);
        assert_eq_f64(test_matrix_3().norm2(), 14.2828568570857);
        assert_eq_f64(test_matrix_4().norm2(), 7.681145747868608);
    }

    #[test]
    fn test_transpose() {
        assert_matrix(&test_matrix_1().transpose().transpose(), &test_matrix_1());
        assert_matrix(&test_matrix_2().transpose().transpose(), &test_matrix_2());
        assert_matrix(&test_matrix_3().transpose().transpose(), &test_matrix_3());
        assert_matrix(&test_matrix_4().transpose().transpose(), &test_matrix_4());
    }

    #[test]
    #[should_panic(expected = "Unable to transpose a non-square matrix in-place.")]
    fn test_transpose_mut_panic() {
        let mut matrix = test_matrix_1();
        matrix.transpose_mut();
    }

    #[test]
    fn test_transpose_mut() {
        let matrix = DoubleMatrix::new_random(3, 3);
        let mut clone = matrix.clone();
        clone.transpose_mut();
        assert_matrix(&clone, &matrix.transpose());
    }

    #[test]
    #[should_panic(expected = "Unable to get the diagonal of a non-square matrix.")]
    fn test_diag_panic() {
        test_matrix_1().diag();
    }

    #[test]
    fn test_diag() {
        let matrix = DoubleMatrix::identity(4, 4);
        assert_matrix(&matrix.diag(), &DoubleMatrix::new(4, 1, vec![1.0, 1.0, 1.0, 1.0]));
    }

    #[test]
    fn test_abs() {
        assert_matrix(&test_matrix_1().abs(), &test_matrix_1());

        let matrix = DoubleMatrix::new(3, 2, vec![
            -1.0, 2.0, -3.0,
            4.0, -5.0, -6.0
        ]);
        let exp = DoubleMatrix::new(3, 2, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0
        ]);
        assert_matrix(&matrix.abs(), &exp);
    }

    #[test]
    fn test_full_svd_test_matrix_2() {
        let a = test_matrix_2();
        let svd = a.full_svd();

        let u_exp = DoubleMatrix::from_row_slice(4, 4, &[
            0.013543, -0.135435, 0.542638, 0.828866,
            0.109341, -0.518419, 0.667345, -0.523390,
            0.470163, -0.714229, -0.481962, 0.191143,
            0.875676, 0.450306, 0.167053, -0.050094
        ]);
        let s_exp = DoubleMatrix::from_row_slice(4, 1, &[
            4.260007, 3.107349, 2.111785, 0.858542
        ]);
        let v_exp = DoubleMatrix::from_row_slice(4, 4, &[
            0.003179, -0.043585, 0.256957, 0.965434,
            0.054513, -0.377258, 0.888977, -0.253819,
            0.356767, -0.856391, -0.368665, 0.058285,
            0.932596, 0.349815, 0.088195, -0.010752
        ]);

        assert_matrix(&a, &test_matrix_2());
        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_full_svd_test_matrix_3() {
        let a = test_matrix_3();
        let svd = a.full_svd();

        let u_exp = DoubleMatrix::from_row_slice(2, 2, &[
            -0.376168, -0.926551,
            -0.926551, 0.376168
        ]);
        let s_exp = DoubleMatrix::from_row_slice(2, 1, &[
            14.227407, 1.257330
        ]);
        let v_exp = DoubleMatrix::from_row_slice(4, 4, &[
            -0.352062, 0.758981, -0.400087, -0.374072,
            -0.443626, 0.321242, 0.254633, 0.796971,
            -0.535190, -0.116498, 0.690996, -0.471724,
            -0.626754, -0.554238, -0.545542, 0.048826
        ]);

        assert_matrix(&a, &test_matrix_3());
        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_full_svd_test_matrix_4() {
        let a = test_matrix_4();
        let svd = a.full_svd();

        let u_exp = DoubleMatrix::from_row_slice(3, 3, &[
            -0.291036, -0.339556, -0.894427,
            -0.582071, -0.679112, 0.447214,
            -0.759270, 0.650776, -0.000000
        ]);
        let s_exp = DoubleMatrix::from_row_slice(2, 1, &[
            7.675619, 0.291321
        ]);
        let v_exp = DoubleMatrix::from_row_slice(2, 2, &[
            -0.486344, 0.873768,
            -0.873768, -0.486344
        ]);

        assert_matrix(&a, &test_matrix_4());
        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_singular_values_matrix_2() {
        let a = test_matrix_2();
        let s = a.singular_values();
        let s_exp = DoubleMatrix::from_row_slice(4, 1, &[
            4.260007, 3.107349, 2.111785, 0.858542
        ]);
        assert_matrix_eps(&s, &s_exp, 1e-6);
    }

    #[test]
    fn test_singular_values_matrix_3() {
        let a = test_matrix_3();
        let s = a.singular_values();
        let s_exp = DoubleMatrix::from_row_slice(2, 1, &[
            14.227407, 1.257330
        ]);
        assert_matrix_eps(&s, &s_exp, 1e-6);
    }

    #[test]
    fn test_svd_matrix_2() {
        let a = test_matrix_2();
        let svd = a.svd(2);

        let u_exp = DoubleMatrix::from_row_slice(4, 2, &[
            -0.013543, -0.135435,
            -0.109341, -0.518419,
            -0.470163, -0.714229,
            -0.875676, 0.450306
        ]);
        let s_exp = DoubleMatrix::from_row_slice(2, 1, &[
            4.260007, 3.107349
        ]);

        let v_exp = DoubleMatrix::from_row_slice(4, 2, &[
            -0.003179, -0.043585,
            -0.054513, -0.377258,
            -0.356767, -0.856391,
            -0.932596, 0.349815
        ]);

        assert_matrix(&a, &test_matrix_2());
        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_lansvd_matrix_2() {
        let a = test_matrix_2();
        let svd = a.lansvd(2);

        let u_exp = DoubleMatrix::from_row_slice(4, 2, &[
            -0.013543, 0.135435,
            -0.109341, 0.518419,
            -0.470163, 0.714229,
            -0.875676, -0.450306
        ]);
        let s_exp = DoubleMatrix::from_row_slice(2, 1, &[
            4.260007, 3.107349
        ]);

        let v_exp = DoubleMatrix::from_row_slice(4, 2, &[
            -0.003179, 0.043585,
            -0.054513, 0.377258,
            -0.356767, 0.856391,
            -0.932596, -0.349815
        ]);

        assert_matrix(&a, &test_matrix_2());
        assert_matrix_eps(&svd.u.unwrap(), &u_exp, 1e-6);
        assert_matrix_eps(&svd.s, &s_exp, 1e-6);
        assert_matrix_eps(&svd.v.unwrap(), &v_exp, 1e-6);
    }

    #[test]
    fn test_matrix_display() {
        let a = DoubleMatrix::from_row_slice(4, 3, &[
            0.010036686563409836, 0.7810703456302204, 111.0, 0.002,
            0.4373140464988601, 123.12, 0.123456, 0.0003,
            0.3285631807472238, 23982.1, 0.01, 0.00004
        ]);

        assert_eq!(
            format!("{:.1$}", a, 3),
"
  .                         .
  |     0.010 0.781 111.000 |
  |     0.002 0.437 123.120 |
  |     0.123 0.000   0.329 |
  | 23982.100 0.010   0.000 |
  .                         .

"
        );
        assert_eq!(
            format!("{}", a),
"
  .                                                            .
  | 0.010036686563409836 0.7810703456302204                111 |
  |                0.002 0.4373140464988601             123.12 |
  |             0.123456             0.0003 0.3285631807472238 |
  |              23982.1               0.01            0.00004 |
  .                                                            .

"
        );

        let a = DoubleMatrix::new(4, 3, vec![
            -0.010036686563409836, -0.7810703456302204, 111.0, 0.002,
            0.4373140464988601, 123.12, -0.123456, -0.0003,
            0.3285631807472238, -23982.1, 0.01, 0.00001298234982344
        ]);

        assert_eq!(
            format!("{:.1$}", a, 3),
"
  .                            .
  |  -0.010   0.437      0.329 |
  |  -0.781 123.120 -23982.100 |
  | 111.000  -0.123      0.010 |
  |   0.002  -0.000      0.000 |
  .                            .

"
        );
        assert_eq!(
            format!("{}", a),
"
  .                                                              .
  | -0.010036686563409836 0.4373140464988601  0.3285631807472238 |
  |   -0.7810703456302204             123.12            -23982.1 |
  |                   111          -0.123456                0.01 |
  |                 0.002            -0.0003 0.00001298234982344 |
  .                                                              .

"
        );
    }
}
