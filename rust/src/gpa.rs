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

//! Module that implements Generalized Procrustes Analysis

use internal::DoubleMatrix;

pub struct GPA {
    mean_shape: DoubleMatrix,
    normal_shapes: Vec<DoubleMatrix>,
    ssd: f64 // Procrustes distance
}

impl GPA {
    pub fn new(mean_shape: DoubleMatrix, normal_shapes: Vec<DoubleMatrix>, ssd: f64) -> Self {
        Self {
            mean_shape: mean_shape,
            normal_shapes: normal_shapes,
            ssd: ssd
        }
    }

    pub fn get_ssd(&self) -> f64 {
        self.ssd
    }

    pub fn get_mean_shape(&self) -> &DoubleMatrix {
        &self.mean_shape
    }

    pub fn get_normal_shapes(&self) -> &[DoubleMatrix] {
        &self.normal_shapes
    }
}

#[inline]
pub fn estimate_gpa(shapes: &[DoubleMatrix], is_scaled: bool, initial_order: usize) -> GPA {
    let (rows, cols) = validate_shapes(shapes);

    let mut normal_shapes: Vec<DoubleMatrix> = vec![];
    let mut local_mean = DoubleMatrix::zeros(shapes.len(), cols);
    for (i, shape) in shapes.iter().enumerate() {
        local_mean.put_row(i, &shape.column_means());
        normal_shapes.push(shape.sub_row_vector(&local_mean.get_row(i)));
    }

    let tol = 1e-3;
    let mut g0 = 1e16;
    let mut g1 = 1e15;

    let mut mean_shape = normal_shapes[initial_order].clone();

    let mut j = 0;
    while g0 - g1 > tol {
        let mut sum_shapes = DoubleMatrix::zeros(rows, cols);
        let mut scale = 1f64;
        for ji in 0..shapes.len() {
            if j == 0 && ji == initial_order {
                sum_shapes.add_matrix_mut(&normal_shapes[ji]);
            } else {
                let tmp = normal_shapes[ji].transpose().mmul(&mean_shape);
                let svd = tmp.full_svd();
                let rotation = svd.u.expect("u").mmul(&svd.v.expect("v").transpose());
                if is_scaled {
                    let trace0 = normal_shapes[ji]
                        .mmul(&rotation)
                        .mmul(&mean_shape.transpose())
                        .diag()
                        .sum();
                    let trace1 = normal_shapes[ji]
                        .mmul(&normal_shapes[ji].transpose())
                        .diag()
                        .sum();
                    scale = trace0 / trace1;
                }
                normal_shapes[ji].mul_scalar_mut(scale);
                normal_shapes[ji].mmul_assign(&rotation);
                sum_shapes.add_matrix_mut(&normal_shapes[ji]);
            }
        }

        sum_shapes.div_scalar_mut(shapes.len() as f64);
        mean_shape = sum_shapes;
        let mean_shape_norm2 = mean_shape.norm2();
        mean_shape.div_scalar_mut(mean_shape_norm2);

        let mut ssd = 0f64;
        for ji in 0..shapes.len() {
            let mut deviation = normal_shapes[ji].sub_matrix(&mean_shape);
            deviation.square_mut();
            ssd += deviation.sum();
        }

        g0 = g1;
        g1 = ssd;
        j += 1;
    }

    GPA::new(mean_shape, normal_shapes, g1)
}

// Validate shapes and return common shape
fn validate_shapes(matrices: &[DoubleMatrix]) -> (usize, usize) {
    // always expect at least one matrix
    assert!(matrices.len() >= 2,
        "Expected at least two matrices, found {} matrices", matrices.len());
    let shape = matrices[0].shape();
    for matrix in matrices {
        assert_eq!(matrix.shape(), shape,
            "Shape dimension mismatch: {:?} != {:?}", matrix.shape(), shape);
    }
    shape
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_gpa() {
        let shapes = vec![
            DoubleMatrix::new_random(4000, 40),
            DoubleMatrix::new_random(4000, 40),
            DoubleMatrix::new_random(4000, 40),
            DoubleMatrix::new_random(4000, 40),
            DoubleMatrix::new_random(4000, 40)
        ];
        let now = Instant::now();
        let gpa = estimate_gpa(&shapes, true, 1);
        let elapsed = now.elapsed();
        println!("gpa.ssd: {}", gpa.get_ssd());
        let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        println!("Took {} ms", sec * 1_000f64);
    }
}
