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
