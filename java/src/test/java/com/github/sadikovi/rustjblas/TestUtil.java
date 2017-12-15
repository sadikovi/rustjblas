package com.github.sadikovi.rustjblas;

import static org.junit.Assert.assertEquals;

/**
 * `TestUtil` class contains methods and properties used for testing.
 * For example, matrix comparison and threshold.
 */
public class TestUtil {
  // comparison threshold
  static final double EPS = 1e-8;

  /** Method to compare jblas matrix with rustjblas matrix */
  static void assertMatrix(org.jblas.DoubleMatrix m, DoubleMatrix n) {
    assertEquals("Matrices are different, row mismatch -", m.rows, n.rows());
    assertEquals("Matrices are different, column mismatch -", m.columns, n.cols());
    double[] a = m.toArray();
    double[] b = n.toArray();
    assertEquals(a.length, b.length);
    for (int i = 0; i < a.length; i++) {
      assertEquals("Matrices are different, value mismatch -", a[i], b[i], EPS);
    }
  }
}
