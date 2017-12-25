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

import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

/**
 * Test suite for matrix properties that are maintenance related.
 */
public class MatrixPropsSuite {
  @Test
  public void testPrettyString() {
    double[] data = new double[]{1.0, 2.0, 3.0, 4.0, 5.0, 6.0};
    DoubleMatrix matrix = DoubleMatrix.fromArray(2, 3, data);
    String expected =
      "  | 1 3 5 |\n" +
      "  | 2 4 6 |\n";
    assertTrue(matrix.prettyString().contains(expected.trim()));
  }

  @Test
  public void testToString() {
    DoubleMatrix matrix = DoubleMatrix.rand(3, 4);
    String pointer = "0x" + Long.toHexString(matrix.ptr()).toUpperCase();
    assertEquals(matrix.toString(), "<matrix 3 x 4 [valid], ptr=" + pointer + ">");

    matrix.dealloc();
    pointer = "0x" + Long.toHexString(-1L).toUpperCase();
    assertEquals(matrix.toString(), "<matrix [dealloc], ptr=" + pointer + ">");
  }

  @Test
  public void testPtr() {
    DoubleMatrix matrix = DoubleMatrix.rand(3, 4);
    assertTrue(matrix.ptr() != -1L);
    matrix.dealloc();
    assertTrue(matrix.ptr() == -1L);
  }

  @Test
  public void testMemoryValid() {
    DoubleMatrix matrix = DoubleMatrix.rand(3, 4);
    assertTrue(matrix.memoryValid());
    matrix.dealloc();
    assertFalse(matrix.memoryValid());
  }
}
