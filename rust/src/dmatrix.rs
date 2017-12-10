use nalgebra::{Dynamic, Matrix, MatrixVec, U1};

// Dynamically sized and allocated float column vector
pub type ColumnVector = Matrix<f32, Dynamic, U1, MatrixVec<f32, Dynamic, U1>>;

// Dynamically sized and allocated float row vector
pub type RowVector = Matrix<f32, U1, Dynamic, MatrixVec<f32, U1, Dynamic>>;

// Dynamically sized and dynamically allocated float matrix
pub type DoubleMatrix = Matrix<f32, Dynamic, Dynamic, MatrixVec<f32, Dynamic, Dynamic>>;
