package com.github.sadikovi;

public class DoubleMatrix {
  private static long INVALID_PTR = -1L;

  private volatile long pointer;

  private DoubleMatrix(long pointer) {
    this.pointer = pointer;
    assert_pointer();
  }

  private synchronized void assert_pointer() {
    if (pointer == INVALID_PTR) {
      throw new IllegalStateException("Invalid state of double matrix, ptr=" + pointer);
    }
  }

  public static DoubleMatrix anew(int rows, int cols, double[] arr) {
    long pointer = alloc_new(rows, cols, arr);
    return new DoubleMatrix(pointer);
  }

  public static DoubleMatrix rand(int rows, int cols) {
    long pointer = alloc_rand(rows, cols);
    return new DoubleMatrix(pointer);
  }

  public static DoubleMatrix zeros(int rows, int cols) {
    long pointer = alloc_zeros(rows, cols);
    return new DoubleMatrix(pointer);
  }

  public static DoubleMatrix ones(int rows, int cols) {
    long pointer = alloc_ones(rows, cols);
    return new DoubleMatrix(pointer);
  }

  public int rows() {
    assert_pointer();
    return matrix_rows(pointer);
  }

  public int cols() {
    assert_pointer();
    return matrix_cols(pointer);
  }

  public void show(boolean truncate) {
    assert_pointer();
    matrix_show(pointer, truncate);
  }

  // truncate output for large matrix
  public void show() {
    show(true);
  }

  public void dealloc() {
    assert_pointer();
    matrix_dealloc(pointer);
    pointer = INVALID_PTR;
  }

  @Override
  public String toString() {
    assert_pointer();
    return matrix_tostring(pointer);
  }

  @Override
  protected void finalize() throws Throwable {
    // might not be ideal to call dealloc in finalize because of some unpredictability of GC
    dealloc();
  }

  // native methods

  private static native long alloc_new(int rows, int cols, double[] arr);

  private static native long alloc_rand(int rows, int cols);

  private static native long alloc_zeros(int rows, int cols);

  private static native long alloc_ones(int rows, int cols);

  private static native int matrix_rows(long pointer);

  private static native int matrix_cols(long pointer);

  private static native void matrix_show(long pointer, boolean truncate);

  private static native String matrix_tostring(long pointer);

  private static native void matrix_dealloc(long pointer);
}
