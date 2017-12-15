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
      "  ┌       ┐\n" +
      "  │ 1 3 5 │\n" +
      "  │ 2 4 6 │\n" +
      "  └       ┘\n";
    assertEquals(matrix.prettyString().trim(), expected.trim());
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
