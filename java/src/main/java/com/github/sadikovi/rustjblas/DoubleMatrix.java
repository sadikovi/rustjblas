package com.github.sadikovi.rustjblas;

/**
 * `DoubleMatrix` class backed by off-heap memory.
 * API resembles one of org.jblas.DoubleMatrix.
 */
public class DoubleMatrix {
  static {
    loadLibrary();
  }

  // Value for invalid pointer, if matrix has invalid pointer it cannot be processed
  private static final long INVALID_PTR = -1L;

  // internal pointer to off-heap memory
  private volatile long pointer;

  /**
   * Create double matrix from off-heap pointer.
   * Pointer should be a valid unsigned int32 value.
   */
  private DoubleMatrix(long pointer) {
    this.pointer = pointer;
    assert_pointer();
  }

  /**
   * Check if current pointer is valid.
   */
  private synchronized void assert_pointer() {
    if (pointer == INVALID_PTR) {
      throw new IllegalStateException("Invalid state of double matrix, ptr=" + pointer);
    }
  }

  // == matrix properties ==

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

  /**
   * Return pretty string for a matrix.
   */
  public String prettyString() {
    assert_pointer();
    return matrix_pretty_string();
  }

  /**
   * Display current matrix in stdout.
   */
  public void show() {
    assert_pointer();
    System.out.println(prettyString());
  }

  /**
   * Get current pointer value (read-only).
   */
  public long ptr() {
    return pointer;
  }

  /**
   * True if matrix is backed by non-invalid pointer, false otherwise.
   */
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
    if (pointer == INVALID_PTR) return "<matrix dealloc, pointer " + pointer + ">";
    return "<matrix valid, pointer " + pointer + ", rows " + rows() + ", columns " + cols() + ">";
  }

  @Override
  protected void finalize() throws Throwable {
    // might not be ideal to call dealloc in finalize because of some unpredictability of GC
    dealloc();
  }

  // == Matrix operations ==

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

  /** Return a vector containing the means of all columns */
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

  // == static methods ==

  /**
   * Create matrix from rows, and columns and on-heap array.
   * New matrix has column-major order.
   */
  public static DoubleMatrix fromArray(int rows, int cols, double[] arr) {
    long pointer = alloc_from_array(rows, cols, arr);
    return new DoubleMatrix(pointer);
  }

  /**
   * Create random matrix for specified dimensions.
   */
  public static DoubleMatrix rand(int rows, int cols) {
    long pointer = alloc_rand(rows, cols);
    return new DoubleMatrix(pointer);
  }

  /**
   * Create matrix of zeros for specified dimensions.
   */
  public static DoubleMatrix zeros(int rows, int cols) {
    long pointer = alloc_zeros(rows, cols);
    return new DoubleMatrix(pointer);
  }

  /**
   * Create matrix of ones for specified dimensions.
   */
  public static DoubleMatrix ones(int rows, int cols) {
    long pointer = alloc_ones(rows, cols);
    return new DoubleMatrix(pointer);
  }

  private static void loadLibrary() {
    // first check LD_LIBRARY_PATH, then DYLD_LIBRARY_PATH
    String linuxPath = System.getenv("LD_LIBRARY_PATH");
    String osxPath = System.getenv("DYLD_LIBRARY_PATH");
    // actual value for library path, append env variables if set
    String value = System.getProperty("java.library.path");
    value = (value == null) ? "." : value;
    if (linuxPath != null) {
      value = value + ":" + linuxPath;
    }
    if (osxPath != null) {
      value = value + ":" + osxPath;
    }
    System.setProperty("java.library.path", value);
    System.out.println("Library path: " + System.getProperty("java.library.path"));
    System.loadLibrary("rustjblas");
  }

  // == native methods ==

  private static native long alloc_from_array(int rows, int cols, double[] arr);
  private static native long alloc_rand(int rows, int cols);
  private static native long alloc_zeros(int rows, int cols);
  private static native long alloc_ones(int rows, int cols);

  private native int matrix_rows();
  private native int matrix_cols();
  private native String matrix_pretty_string();
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

  private native long matrix_column_mins();
  private native long matrix_column_maxs();
  private native long matrix_column_means();
  private native long matrix_column_sums();

  private native double matrix_min();
  private native double matrix_max();
  private native double matrix_sum();
  private native double matrix_norm1();
  private native double matrix_norm2();

  private native long matrix_transpose();
  private native long matrix_diag();
}
