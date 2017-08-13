package com.github.sadikovi;

public class DoubleMatrix {
  static {
    loadLibrary();
  }

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

  public int rows() {
    assert_pointer();
    return matrix_rows();
  }

  public int cols() {
    assert_pointer();
    return matrix_cols();
  }

  public void show(boolean truncate) {
    assert_pointer();
    matrix_show(truncate);
  }

  // truncate output for large matrix
  public void show() {
    show(true);
  }

  public void dealloc() {
    assert_pointer();
    matrix_dealloc();
    pointer = INVALID_PTR;
  }

  // get current pointer value
  public long ptr() {
    return pointer;
  }

  // whether or not matrix is valid (not deallocated)
  public boolean memoryValid() {
    return pointer != INVALID_PTR;
  }

  @Override
  public String toString() {
    if (pointer == INVALID_PTR) return "<matrix dealloc, pointer " + pointer + ">";
    return matrix_tostring();
  }

  @Override
  protected void finalize() throws Throwable {
    // might not be ideal to call dealloc in finalize because of some unpredictability of GC
    dealloc();
  }

  // static methods

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

  // native methods

  private static native long alloc_new(int rows, int cols, double[] arr);

  private static native long alloc_rand(int rows, int cols);

  private static native long alloc_zeros(int rows, int cols);

  private static native long alloc_ones(int rows, int cols);

  private native int matrix_rows();

  private native int matrix_cols();

  private native void matrix_show(boolean truncate);

  private native String matrix_tostring();

  private native void matrix_dealloc();
}
