extern crate rustjblas;

use rustjblas::matrix::DoubleMatrix;

fn main() {
    let m = DoubleMatrix::new(3, 3, vec![1.0, 4.0, 7.0, 4.0, 5.0, 8.0, 7.0, 8.0, 9.0]);
    println!("{}", m);
    println!("is_square: {}", m.is_square());
    println!("is_symmetric: {}", m.is_symmetric());
    println!("is_vector: {}", m.is_vector());
    println!("is_row_vector: {}", m.is_row_vector());
    println!("is_column_vector: {}", m.is_column_vector());
    println!("{:?}", m);
}
