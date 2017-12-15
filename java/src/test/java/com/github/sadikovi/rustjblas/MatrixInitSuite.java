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
