/*
 * Copyright (c) 2017 sadikovi
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

package com.github.sadikovi.rustjblas;

/**
 * `DoubleMatrix` class backed by off-heap memory.
 * API resembles one of org.jblas.DoubleMatrix.
 *
 * Most of the methods should be expected to throw
 * `com.github.sadikovi.rustjblas.OperationException`.
 */
public class DoubleMatrix {
  static {
    loadLibrary("rustjblas");
  }

  // Value for invalid pointer, if matrix has invalid pointer it cannot be processed
  private static final long INVALID_PTR = -1L;

  // internal pointer to off-heap memory
  private volatile long pointer;

  /**
   * Create matrix from rows, and columns and on-heap array.
   * New matrix has column-major order, thus array is expected to be in column-major order.
   */
  public static DoubleMatrix fromArray(int rows, int cols, double[] arr) {
    assert_shape(rows, cols);
    if (arr == null || arr.length != rows * cols) {
      throw new IllegalArgumentException("Invalid data array: " + arr);
    }
    long pointer = alloc_from_array(rows, cols, arr);
    return new DoubleMatrix(pointer);
  }

  /**
   * Create matrix from a set of row arrays.
   * Arrays should have equal length.
   */
  public static DoubleMatrix fromRows(double[][] values) {
    int rows = values.length;
    int cols = (rows > 0) ? values[0].length : 0;

    // assert number of columns in each row
    for (int i = 0; i < rows; i++) {
      if (cols != values[i].length) {
        throw new IllegalArgumentException("Invalid number of columns in row " + (i + 1) +
          " (" + cols + " != " + values[i].length + ")");
      }
    }

    double[] arr = new double[rows * cols];
    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < cols; j++) {
        arr[i + j*rows] = values[i][j];
      }
    }
    return fromArray(rows, cols, arr);
  }

  /** Create random matrix for specified dimensions */
  public static DoubleMatrix rand(int rows, int cols) {
    assert_shape(rows, cols);
    long pointer = alloc_rand(rows, cols);
    return new DoubleMatrix(pointer);
  }

  /** Create matrix of zeros for specified dimensions */
  public static DoubleMatrix zeros(int rows, int cols) {
    assert_shape(rows, cols);
    long pointer = alloc_zeros(rows, cols);
    return new DoubleMatrix(pointer);
  }

  /** Create matrix of ones for specified dimensions */
  public static DoubleMatrix ones(int rows, int cols) {
    assert_shape(rows, cols);
    long pointer = alloc_ones(rows, cols);
    return new DoubleMatrix(pointer);
  }

  /** Construct a new n-by-n identity matrix */
  public static DoubleMatrix eye(int n) {
    assert_shape(n, n);
    long pointer = alloc_identity(n, n);
    return new DoubleMatrix(pointer);
  }

  /** Load implementation library */
  private static void loadLibrary(String libname) {
    System.out.println("-- Loading library " + libname + ", libpath: " +
      System.getProperty("java.library.path"));
    System.loadLibrary(libname);
  }

  /**
   * Create double matrix from off-heap pointer.
   * Pointer should be a valid unsigned int32 value.
   */
  private DoubleMatrix(long pointer) {
    this.pointer = pointer;
    assert_pointer();
  }

  /** Convert pointer into hex string */
  private static String ptrstr(long pointer) {
    return "0x" + Long.toHexString(pointer).toUpperCase();
  }

  /** Check if current pointer is valid */
  private synchronized void assert_pointer() {
    if (pointer == INVALID_PTR) {
      throw new IllegalStateException("Invalid state of double matrix, ptr=" + ptrstr(pointer));
    }
  }

  /** Assert matrix shape */
  private static void assert_shape(int rows, int cols) {
    if (rows < 0 || cols < 0) {
      throw new IllegalArgumentException("Invalid matrix shape: rows " + rows + ", cols " + cols);
    }
  }

  /**
   * Return pretty string for a matrix
   * When `truncate` is `true`, values are shown up to a certain precision to fit the screen,
   * otherwise, when `truncate` is `false`, full values are shown.
   */
  public String prettyString(boolean truncate) {
    assert_pointer();
    // 1 - truncated values, 0 - full values
    return matrix_pretty_string(truncate ? 1 : 0);
  }

  /** Return pretty string for a matrix with truncated values */
  public String prettyString() {
    return prettyString(true);
  }

  /**
   * Display current matrix in stdout.
   * If `truncate` is `false`, full precision values are shown, otherwise values are truncated. 
   */
  public void show(boolean truncate) {
    assert_pointer();
    System.out.println(prettyString(truncate));
  }

  /** Display current matrix with truncated values in stdout */
  public void show() {
    show(true);
  }

  /** Get current pointer value (read-only) */
  public long ptr() {
    return pointer;
  }

  /** True if matrix is backed by a valid pointer, false otherwise */
  public boolean memoryValid() {
    return pointer != INVALID_PTR;
  }

  /**
   * Deallocate off-heap storage for the matrix.
   * Matrix is invalid and cannot be used after this operation.
   */
  public void dealloc() {
    assert_pointer();
    matrix_dealloc();
    pointer = INVALID_PTR;
  }

  @Override
  public String toString() {
    if (pointer == INVALID_PTR) return "<matrix [dealloc], ptr=" + ptrstr(pointer) + ">";
    return "<matrix " + rows() + " x " + cols() + " [valid], ptr=" + ptrstr(pointer) + ">";
  }

  @Override
  protected void finalize() throws Throwable {
    // might not be ideal to call dealloc in finalize because of some unpredictability of GC
    try {
      if (memoryValid()) {
        dealloc();
      }
    } finally {
      super.finalize();
    }
  }

  // == Matrix operations ==

  /** Return number of rows in the matrix */
  public int rows() {
    assert_pointer();
    return matrix_rows();
  }

  /** Return number of columns in the matrix */
  public int cols() {
    assert_pointer();
    return matrix_cols();
  }

  /** Return a copy of matrix data array in column-major order */
  public double[] toArray() {
    assert_pointer();
    long rows = rows(), cols = cols();
    if (rows * cols >= Integer.MAX_VALUE) {
      throw new IllegalStateException("Cannot convert matrix " +
        rows + "x" + cols + " to array, too large");
    }
    return matrix_data_array();
  }

  /** Add scalar value to this matrix */
  public DoubleMatrix add(double scalar) {
    assert_pointer();
    long res = matrix_add_scalar(scalar);
    return new DoubleMatrix(res);
  }

  /** Add matrix to this matrix */
  public DoubleMatrix add(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long res = matrix_add_matrix(that.ptr());
    return new DoubleMatrix(res);
  }

  /** Add scalar value to this matrix (in-place) */
  public DoubleMatrix addi(double scalar) {
    assert_pointer();
    matrix_add_in_place_scalar(scalar);
    return this;
  }

  /** Add matrix to this matrix (in-place) */
  public DoubleMatrix addi(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_add_in_place_matrix(that.ptr());
    return this;
  }

  /** Subtract a scalar from this matrix */
  public DoubleMatrix sub(double scalar) {
    assert_pointer();
    long res = matrix_sub_scalar(scalar);
    return new DoubleMatrix(res);
  }

  /** Subtract a matrix */
  public DoubleMatrix sub(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long res = matrix_sub_matrix(that.ptr());
    return new DoubleMatrix(res);
  }

  /** Subtract a scalar from this matrix (in-place) */
  public DoubleMatrix subi(double scalar) {
    assert_pointer();
    matrix_sub_in_place_scalar(scalar);
    return this;
  }

  /** Subtract a matrix (in-place) */
  public DoubleMatrix subi(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_sub_in_place_matrix(that.ptr());
    return this;
  }

  /** Elementwise multiply by a scalar */
  public DoubleMatrix mul(double scalar) {
    assert_pointer();
    long res = matrix_mul_scalar(scalar);
    return new DoubleMatrix(res);
  }

  /** Elementwise multiply by a matrix */
  public DoubleMatrix mul(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long res = matrix_mul_matrix(that.ptr());
    return new DoubleMatrix(res);
  }

  /** Elementwise multiply by a scalar (in-place) */
  public DoubleMatrix muli(double scalar) {
    assert_pointer();
    matrix_mul_in_place_scalar(scalar);
    return this;
  }

  /** Elementwise multiply by a matrix (in-place) */
  public DoubleMatrix muli(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_mul_in_place_matrix(that.ptr());
    return this;
  }

  /** Elementwise divide by a scalar */
  public DoubleMatrix div(double scalar) {
    assert_pointer();
    long res = matrix_div_scalar(scalar);
    return new DoubleMatrix(res);
  }

  /** Elementwise divide by a matrix */
  public DoubleMatrix div(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long res = matrix_div_matrix(that.ptr());
    return new DoubleMatrix(res);
  }

  /** Elementwise divide by a scalar (in-place) */
  public DoubleMatrix divi(double scalar) {
    assert_pointer();
    matrix_div_in_place_scalar(scalar);
    return this;
  }

  /** Elementwise divide by a matrix (in place). */
  public DoubleMatrix divi(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_div_in_place_matrix(that.ptr());
    return this;
  }

  /** Matrix-multiply by a matrix */
  public DoubleMatrix mmul(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long res = matrix_mmul_matrix(that.ptr());
    return new DoubleMatrix(res);
  }

  /** Matrix-multiply by a matrix (in place) */
  public DoubleMatrix mmuli(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_mmul_in_place_matrix(that.ptr());
    return this;
  }

  /** Return column-wise minimums */
  public DoubleMatrix columnMins() {
    assert_pointer();
    long res = matrix_column_mins();
    return new DoubleMatrix(res);
  }

  /** Return column-wise maximums */
  public DoubleMatrix columnMaxs() {
    assert_pointer();
    long res = matrix_column_maxs();
    return new DoubleMatrix(res);
  }

  /** Return a vector containing the means of the columns */
  public DoubleMatrix columnMeans() {
    assert_pointer();
    long res = matrix_column_means();
    return new DoubleMatrix(res);
  }

  /** Return a vector containing the sums of the columns */
  public DoubleMatrix columnSums() {
    assert_pointer();
    long res = matrix_column_sums();
    return new DoubleMatrix(res);
  }

  /** Return row-wise minimums */
  public DoubleMatrix rowMins() {
    assert_pointer();
    long res = matrix_row_mins();
    return new DoubleMatrix(res);
  }

  /** Return row-wise maximums */
  public DoubleMatrix rowMaxs() {
    assert_pointer();
    long res = matrix_row_maxs();
    return new DoubleMatrix(res);
  }

  /** Return a vector containing the means of the rows */
  public DoubleMatrix rowMeans() {
    assert_pointer();
    long res = matrix_row_means();
    return new DoubleMatrix(res);
  }

  /** Return a vector containing the sums of the rows */
  public DoubleMatrix rowSums() {
    assert_pointer();
    long res = matrix_row_sums();
    return new DoubleMatrix(res);
  }

  /** Put (update) column in this matrix with a column vector */
  public void putColumn(int col, DoubleMatrix vector) {
    assert_pointer();
    vector.assert_pointer();
    matrix_put_column(col, vector.ptr());
  }

  /** Get column as vector for the column index */
  public DoubleMatrix getColumn(int col) {
    assert_pointer();
    long res = matrix_get_column(col);
    return new DoubleMatrix(res);
  }

  /** Put (update) row in this matrix with a row vector */
  public void putRow(int row, DoubleMatrix vector) {
    assert_pointer();
    vector.assert_pointer();
    matrix_put_row(row, vector.ptr());
  }

  /** Get row as vector for the row index */
  public DoubleMatrix getRow(int row) {
    assert_pointer();
    long res = matrix_get_row(row);
    return new DoubleMatrix(res);
  }

  /** Return the minimal element of this matrix */
  public double min() {
    assert_pointer();
    return matrix_min();
  }

  /** Return the maximal element of this matrix */
  public double max() {
    assert_pointer();
    return matrix_max();
  }

  /** Compute the sum of all elements of this matrix */
  public double sum() {
    assert_pointer();
    return matrix_sum();
  }

  /** The 1-norm of the matrix as vector (sum of absolute values of elements) */
  public double norm1() {
    assert_pointer();
    return matrix_norm1();
  }

  /** The Euclidean norm of the matrix as vector, also the Frobenius norm of the matrix */
  public double norm2() {
    assert_pointer();
    return matrix_norm2();
  }

  /** Return transposed copy of this matrix */
  public DoubleMatrix transpose() {
    assert_pointer();
    long res = matrix_transpose();
    return new DoubleMatrix(res);
  }

  /** Return the diagonal of the matrix */
  public DoubleMatrix diag() {
    assert_pointer();
    long res = matrix_diag();
    return new DoubleMatrix(res);
  }

  /** Return copy of this matrix with all absolute values */
  public DoubleMatrix abs() {
    assert_pointer();
    long res = matrix_abs();
    return new DoubleMatrix(res);
  }

  // == singular value decomposition methods ==

  /** Placeholder class to store SVD result */
  private static class SvdResult {
    long u = INVALID_PTR;
    long s = INVALID_PTR;
    long v = INVALID_PTR;
  }

  /**
   * Compute full SVD with optional U and V.
   *
   * Use this method when most of the singular values are required, it will be faster than `svd(k)`
   * method.
   */
  public DoubleMatrix[] fullSVD() {
    assert_pointer();
    // store U, s, V
    SvdResult ptrs = new SvdResult();
    matrix_full_svd(ptrs);
    DoubleMatrix[] res = new DoubleMatrix[3];
    res[0] = (ptrs.u == INVALID_PTR) ? null : new DoubleMatrix(ptrs.u);
    res[1] = (ptrs.s == INVALID_PTR) ? null : new DoubleMatrix(ptrs.s);
    res[2] = (ptrs.v == INVALID_PTR) ? null : new DoubleMatrix(ptrs.v);
    return res;
  }

  /** Compute singular values for this matrix and return them as column vector */
  public DoubleMatrix singularValues() {
    assert_pointer();
    long res = matrix_singular_values();
    return new DoubleMatrix(res);
  }

  /**
   * Compute SVD for the first k top singular values.
   *
   * Works fast on relatively small matrices or rows > cols, it is slower when matrix is
   * square, but works on any matrix and k.
   * When k is large (close to min(rows, cols)) consider using `fullSVD()` method.
   *
   * Works great for k >= 50. If k is smaller consider using `lansvd()`.
   *
   * Based on DGESVDX that uses an eigenvalue problem for obtaining the SVD, which allows for the
   * computation of a subset of singular values and vectors. See Lapack/DBDSVDX for details.
   * This method trades performance for high accuracy of computing singular values.
   */
  public DoubleMatrix[] svd(int k) {
    assert_pointer();
    // store U, s, V
    SvdResult ptrs = new SvdResult();
    matrix_svd_k(ptrs, k);
    DoubleMatrix[] res = new DoubleMatrix[3];
    res[0] = (ptrs.u == INVALID_PTR) ? null : new DoubleMatrix(ptrs.u);
    res[1] = (ptrs.s == INVALID_PTR) ? null : new DoubleMatrix(ptrs.s);
    res[2] = (ptrs.v == INVALID_PTR) ? null : new DoubleMatrix(ptrs.v);
    return res;
  }

  /**
   * Compute SVD for the first k top singular values.
   *
   * Currently computes both left and right singular vectors, this behaviour can be changed to
   * allow faster computation. Trades accuracy for faster performance, but should be reasonably
   * accurate within 1e-8.
   *
   * Shows great performance for k < 50. For k larger than that performance starts degrading, so it
   * may be worth considering `svd()` method instead.
   *
   * Based on DLANSVD_IRL that computes the leading singular triplets of a large and sparse matrix
   * A by implicitly restarted Lanczos bidiagonalization with partial reorthogonalization.
   * Note that current method uses A as dense matrix, which is influenced by using dense_matmul,
   * based on BLAS dgemv.
   */
  public DoubleMatrix[] lansvd(int k) {
    assert_pointer();
    // store U, s, V
    SvdResult ptrs = new SvdResult();
    matrix_lansvd_k(ptrs, k);
    DoubleMatrix[] res = new DoubleMatrix[3];
    res[0] = (ptrs.u == INVALID_PTR) ? null : new DoubleMatrix(ptrs.u);
    res[1] = (ptrs.s == INVALID_PTR) ? null : new DoubleMatrix(ptrs.s);
    res[2] = (ptrs.v == INVALID_PTR) ? null : new DoubleMatrix(ptrs.v);
    return res;
  }

  // == native methods ==

  private static native long alloc_from_array(int rows, int cols, double[] arr);
  private static native long alloc_rand(int rows, int cols);
  private static native long alloc_zeros(int rows, int cols);
  private static native long alloc_ones(int rows, int cols);
  private static native long alloc_identity(int rows, int cols);

  private native int matrix_rows();
  private native int matrix_cols();
  private native double[] matrix_data_array();
  private native String matrix_pretty_string(int truncate);
  private native void matrix_dealloc();

  private native long matrix_add_scalar(double scalar);
  private native long matrix_add_matrix(long ptr);
  private native void matrix_add_in_place_scalar(double scalar);
  private native void matrix_add_in_place_matrix(long ptr);

  private native long matrix_sub_scalar(double scalar);
  private native long matrix_sub_matrix(long ptr);
  private native void matrix_sub_in_place_scalar(double scalar);
  private native void matrix_sub_in_place_matrix(long ptr);

  private native long matrix_mul_scalar(double scalar);
  private native long matrix_mul_matrix(long ptr);
  private native void matrix_mul_in_place_scalar(double scalar);
  private native void matrix_mul_in_place_matrix(long ptr);

  private native long matrix_div_scalar(double scalar);
  private native long matrix_div_matrix(long ptr);
  private native void matrix_div_in_place_scalar(double scalar);
  private native void matrix_div_in_place_matrix(long ptr);

  private native long matrix_mmul_matrix(long ptr);
  private native void matrix_mmul_in_place_matrix(long ptr);

  private native long matrix_column_mins();
  private native long matrix_column_maxs();
  private native long matrix_column_means();
  private native long matrix_column_sums();

  private native long matrix_row_mins();
  private native long matrix_row_maxs();
  private native long matrix_row_means();
  private native long matrix_row_sums();

  private native void matrix_put_column(int col, long ptr);
  private native long matrix_get_column(int col);
  private native void matrix_put_row(int row, long ptr);
  private native long matrix_get_row(int row);

  private native double matrix_min();
  private native double matrix_max();
  private native double matrix_sum();
  private native double matrix_norm1();
  private native double matrix_norm2();

  private native long matrix_transpose();
  private native long matrix_diag();
  private native long matrix_abs();

  private native void matrix_full_svd(SvdResult ptrs);
  private native long matrix_singular_values();
  private native void matrix_svd_k(SvdResult ptrs, int k);
  private native void matrix_lansvd_k(SvdResult ptrs, int k);
}
