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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_sums() {
        let matrix = DoubleMatrix::from_row_slice(4, 3, &[
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
            10.0, 11.0, 12.0
        ]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[22.0, 26.0, 30.0]);
        let res = column_sums(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_sums_single_row() {
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let res = column_sums(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_sums_single_column() {
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[6.0]);
        let res = column_sums(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_sums_single_element() {
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let res = column_sums(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_mins() {
        let matrix = DoubleMatrix::from_row_slice(4, 3, &[
            1.0, 2.0, 3.0,
            10.0, 11.0, 12.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_mins_single_row() {
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_mins_single_column() {
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[1.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_mins_single_element() {
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_maxs() {
        let matrix = DoubleMatrix::from_row_slice(4, 3, &[
            1.0, 2.0, 3.0,
            10.0, 11.0, 12.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[10.0, 11.0, 12.0]);
        let res = column_maxs(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_maxs_single_row() {
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_maxs_single_column() {
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[2.0, 1.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let res = column_maxs(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_maxs_single_element() {
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let res = column_maxs(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_means() {
        let matrix = DoubleMatrix::from_row_slice(4, 3, &[
            1.0, 2.0, 3.0,
            10.0, 11.0, 12.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        ]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[5.5, 6.5, 7.5]);
        let res = column_means(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_means_single_row() {
        let matrix = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 3, &[1.0, 2.0, 3.0]);
        let res = column_mins(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_means_single_column() {
        let matrix = DoubleMatrix::from_row_slice(3, 1, &[21.0, 12.0, 36.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[23.0]);
        let res = column_means(&matrix);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_column_means_single_element() {
        let matrix = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let exp = DoubleMatrix::from_row_slice(1, 1, &[3.0]);
        let res = column_means(&matrix);
        assert_eq!(res, exp);
    }
}
