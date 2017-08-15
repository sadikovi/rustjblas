extern crate rsjblas;

use rsjblas::matrix::DoubleMatrix;

fn main() {
    let m = DoubleMatrix::rand(6, 3);
    println!("{}", m);
    println!("is_square: {}", m.is_square());
    println!("is_symmetric: {}", m.is_symmetric());
    println!("is_vector: {}", m.is_vector());
    println!("is_row_vector: {}", m.is_row_vector());
    println!("is_column_vector: {}", m.is_column_vector());
    println!("{:?}", m);
    println!("{}", format!("'{:>width$}'", "abc", width = 10));
    m.show(true);

    // try put() method
    let mut m = DoubleMatrix::ones(3, 2);
    m.put(0, 1, 5.0);
    m.show(true);
}
