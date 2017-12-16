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
import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import static com.github.sadikovi.rustjblas.TestUtil.EPS;

/**
 * Test suite for matrix init methods.
 */
public class MatrixInitSuite {
  // == DoubleMatrix.fromArray ==

  @Test(expected = IllegalArgumentException.class)
  public void testFromArrayInvalidRows() {
    DoubleMatrix.fromArray(-1, 2, new double[4]);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testFromArrayInvalidCols() {
    DoubleMatrix.fromArray(1, -2, new double[4]);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testFromArrayInvalidShape() {
    DoubleMatrix.fromArray(-1, -2, new double[4]);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testFromArrayNullArray() {
    DoubleMatrix.fromArray(2, 2, null);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testFromArrayInvalidArray() {
    DoubleMatrix.fromArray(2, 2, new double[5]);
  }

  @Test
  public void testFromArray() {
    double[] data = new double[]{1.0, 2.0, 3.0, 4.0, 5.0, 6.0};
    DoubleMatrix matrix = DoubleMatrix.fromArray(2, 3, data);
    assertEquals(matrix.rows(), 2);
    assertEquals(matrix.cols(), 3);
    assertArrayEquals(matrix.toArray(), data, EPS);
    matrix.dealloc();
  }

  // == DoubleMatrix.rand ==

  @Test(expected = IllegalArgumentException.class)
  public void testRandInvalidRows() {
    DoubleMatrix.rand(-1, 2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testRandInvalidCols() {
    DoubleMatrix.rand(1, -2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testRandInvalidShape() {
    DoubleMatrix.rand(-1, -2);
  }

  @Test
  public void testRand() {
    DoubleMatrix matrix = DoubleMatrix.rand(3, 4);
    assertEquals(matrix.rows(), 3);
    assertEquals(matrix.cols(), 4);
    for (double elem : matrix.toArray()) {
      assertTrue(elem > 0 && elem < 1);
    }
    matrix.dealloc();
  }

  // == DoubleMatrix.zeros ==

  @Test(expected = IllegalArgumentException.class)
  public void testZerosInvalidRows() {
    DoubleMatrix.zeros(-1, 2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testZerosInvalidCols() {
    DoubleMatrix.zeros(1, -2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testZerosInvalidShape() {
    DoubleMatrix.zeros(-1, -2);
  }

  @Test
  public void testZeros() {
    DoubleMatrix matrix = DoubleMatrix.zeros(3, 4);
    assertEquals(matrix.rows(), 3);
    assertEquals(matrix.cols(), 4);
    for (double elem : matrix.toArray()) {
      assertEquals(elem, 0.0, EPS);
    }
    matrix.dealloc();
  }

  // == DoubleMatrix.ones ==

  @Test(expected = IllegalArgumentException.class)
  public void testOnesInvalidRows() {
    DoubleMatrix.ones(-1, 2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testOnesInvalidCols() {
    DoubleMatrix.ones(1, -2);
  }

  @Test(expected = IllegalArgumentException.class)
  public void testOnesInvalidShape() {
    DoubleMatrix.ones(-1, -2);
  }

  @Test
  public void testOnes() {
    DoubleMatrix matrix = DoubleMatrix.ones(3, 4);
    assertEquals(matrix.rows(), 3);
    assertEquals(matrix.cols(), 4);
    for (double elem : matrix.toArray()) {
      assertEquals(elem, 1.0, EPS);
    }
    matrix.dealloc();
  }
}
