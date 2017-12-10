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
   * Display current matrix in stdout.
   * If truncate is true, only part of the values is displayed.
   */
  public void show(boolean truncate) {
    assert_pointer();
    matrix_show(truncate);
  }

  /**
   * Show matrix (truncate output if matrix is large).
   */
  public void show() {
    show(true);
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

  /**
   * Add scalar value to the matrix.
   * Returns new matrix with added value.
   */
  public DoubleMatrix add(double scalar) {
    assert_pointer();
    long newPointer = matrix_add_scalar(scalar);
    return new DoubleMatrix(newPointer);
  }

  /**
   * Add DoubleMatrix to the matrix.
   * Returns new matrix with added values.
   */
  public DoubleMatrix add(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    long newPointer = matrix_add_matrix(that.ptr());
    return new DoubleMatrix(newPointer);
  }

  /** Add scalar value to the matrix (in-place) */
  public DoubleMatrix addi(double scalar) {
    assert_pointer();
    matrix_add_in_place_scalar(scalar);
    return this;
  }

  /**
   * Add DoubleMatrix to the matrix (in-place).
   * Only this matrix is changed.
   */
  public DoubleMatrix addi(DoubleMatrix that) {
    assert_pointer();
    that.assert_pointer();
    matrix_add_in_place_matrix(that.ptr());
    return this;
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
    System.loadLibrary(value);
  }

  // == native methods ==

  private static native long alloc_from_array(int rows, int cols, double[] arr);
  private static native long alloc_rand(int rows, int cols);
  private static native long alloc_zeros(int rows, int cols);
  private static native long alloc_ones(int rows, int cols);

  private native int matrix_rows();
  private native int matrix_cols();
  private native void matrix_show(boolean truncate);
  private native void matrix_dealloc();

  private native long matrix_add_scalar(double scalar);
  private native long matrix_add_matrix(long ptr);
  private native void matrix_add_in_place_scalar(double scalar);
  private native void matrix_add_in_place_matrix(long ptr);
}
