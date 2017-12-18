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

#![feature(test)]

extern crate test;
extern crate rustjblas;

use rustjblas::matrix;
use rustjblas::matrix::DoubleMatrix;
use test::Bencher;

#[bench]
fn bench_matrix_mul_orig(b: &mut Bencher) {
    let m1 = DoubleMatrix::new_random(100, 100);
    let m2 = DoubleMatrix::new_random(100, 100);
    b.iter(|| &m1 * &m2);
}

#[bench]
fn bench_matrix_mul_blas(b: &mut Bencher) {
    let m1 = DoubleMatrix::new_random(100, 100);
    let m2 = DoubleMatrix::new_random(100, 100);
    b.iter(|| matrix::mmul(&m1, &m2));
}

#[bench]
fn bench_create_random_matrix_orig(b: &mut Bencher) {
    b.iter(|| DoubleMatrix::new_random(100, 100));
}

#[bench]
fn bench_create_random_matrix_tmp(b: &mut Bencher) {
    b.iter(|| matrix::new_random(100, 100));
}
