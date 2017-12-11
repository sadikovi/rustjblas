use nalgebra::{Dynamic, Matrix, MatrixVec};

// Dynamically sized and dynamically allocated float matrix
pub type DoubleMatrix = Matrix<f64, Dynamic, Dynamic, MatrixVec<f64, Dynamic, Dynamic>>;
