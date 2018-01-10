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

//! Lanczos algorithm with selective orthogonalization Using Simon's Recurrence
use std::f64;
use internal::{DoubleMatrix, SVD};

// Validate input parameters
// Parameters:
// - `matrix` matrix to check
// - `endl` left end of interval containing unwanted eigenvalues of B
// - `endr` right end of interval containing unwanted eigenvalues of B
// - `dimensions` upper limit of desired number of eigenpairs of B
// - `iterations` upper limit of desired number of lanczos steps
fn check_parameters(
    matrix: &DoubleMatrix,
    endl: f64,
    endr: f64,
    dimensions: usize,
    iterations: usize
)
{
  if endl > endr {
    panic!("endl ({}) must be less than endr ({})", endl, endr);
  }

  if matrix.rows() == 0 || matrix.cols() == 0 {
    panic!("One of the matrix dimensions is equal to zero");
  }

  if iterations == 0 || iterations > matrix.cols() || iterations > matrix.rows() {
    panic!("Num iterations (number of lanczos steps) is invalid");
  }

  if dimensions == 0 {
    panic!("Requested dimensions (number of eigenpairs desired) is invalid");
  }

  if dimensions > iterations {
    panic!("Requested dimensions ({}) cannot exceed num iterations ({})", dimensions, iterations);
  }
}

// Compute SVD using las2 function
#[allow(dead_code)]
pub fn svd_las2(a: &DoubleMatrix, dimensions: usize) -> SVD {
    svd_las2_k(a, dimensions, 1e-6)
}

// Compute SVD using las2 function with configurable accuracy of ritz values acceptable as
// singular values of the matrix
#[allow(dead_code)]
pub fn svd_las2_k(a: &DoubleMatrix, dimensions: usize, kappa: f64) -> SVD {
    let endl: f64 = -1.0e-30;
    let endr: f64 = 1.0e-30;
    // select number of iterations as a number of dimensions required
    let iterations = dimensions;
    las2(a, dimensions, iterations,  endl, endr, kappa)
}

// LAS2 driver function that checks input parameters, determines machine constants, makes Lanczos
// run and calculates B-eignevectors (singular vectors of A).
// Parameters:
// - `a` input matrix
// - `dimensions` upper limit of desired number of eigenpairs
// - `iterations` upper limit of desired number of Lanczos steps
// - `endl` left end of interval containing unwanted eigenvalues of B
// - `endr` right end of interval containing unwanted eigenvalues of B
// - `kappa` relative accuracy of ritz values acceptable as eigenvalues of B (singular values of A)
fn las2(
    a: &DoubleMatrix,
    dimensions: usize,
    iterations: usize,
    endl: f64,
    endr: f64,
    kappa: f64
) -> SVD
{
    // check input parameters
    check_parameters(a, endl, endr, dimensions, iterations);
    // if matrix is wide, the SVD is computed on its transpose for better performance
    // also perform a copy of matrix, transpose returns copy as well
    let a: DoubleMatrix = if a.cols() > a.rows() { a.transpose() } else { a.clone() };

    let eps = f64::EPSILON;
    let reps = eps.sqrt();
    let eps34 = reps * reps.sqrt();

    unimplemented!()
}
