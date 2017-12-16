package com.github.sadikovi.rustjblas;

import org.junit.Test;
import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import static com.github.sadikovi.rustjblas.TestUtil.EPS;
import static com.github.sadikovi.rustjblas.TestUtil.assertMatrix;

/**
 * Test suite for matrix operations.
 */
public class MatrixOperationsSuite {
  @Test
  public void testShape() {
    DoubleMatrix matrix = DoubleMatrix.rand(0, 0);
    assertEquals(matrix.rows(), 0);
    assertEquals(matrix.cols(), 0);
    matrix.dealloc();

    matrix = DoubleMatrix.rand(1, 1);
    assertEquals(matrix.rows(), 1);
    assertEquals(matrix.cols(), 1);
    matrix.dealloc();

    matrix = DoubleMatrix.rand(32, 24);
    assertEquals(matrix.rows(), 32);
    assertEquals(matrix.cols(), 24);
    matrix.dealloc();
  }

  @Test
  public void testDataArray() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(11, 21);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertArrayEquals(m.toArray(), n.toArray(), EPS);
    n.dealloc();

    m = org.jblas.DoubleMatrix.ones(11, 21);
    n = DoubleMatrix.ones(11, 21);
    assertArrayEquals(m.toArray(), n.toArray(), EPS);
    n.dealloc();

    m = org.jblas.DoubleMatrix.zeros(11, 21);
    n = DoubleMatrix.zeros(11, 21);
    assertArrayEquals(m.toArray(), n.toArray(), EPS);
    n.dealloc();
  }

  // == Matrix elementwise addition ==

  @Test
  public void testAddScalar() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(11, 21);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.add(12.345), n.add(12.345));
    n.dealloc();
  }

  @Test
  public void testAddMatrix() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(11, 21);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.add(m), n.add(n));
    n.dealloc();

    m = org.jblas.DoubleMatrix.rand(11, 21);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(11, 21);
    n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.add(ma), n.add(na));
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testAddMatrixFail() {
    DoubleMatrix n = DoubleMatrix.rand(10, 20);
    n.add(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  @Test
  public void testAddScalarInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(11, 21);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    m.addi(12.345);
    n.addi(12.345);
    assertMatrix(m, n);
    n.dealloc();
  }

  @Test
  public void testAddMatrixInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(11, 21);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(11, 21);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    m.addi(ma);
    n.addi(na);
    assertMatrix(m, n);
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testAddMatrixInPlaceFail() {
    DoubleMatrix n = DoubleMatrix.rand(10, 20);
    n.addi(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  // == Matrix elementwise subtraction ==

  @Test
  public void testSubScalar() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(20, 40);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.sub(4.5), n.sub(4.5));
    n.dealloc();
  }

  @Test
  public void testSubScalarInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(20, 40);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    m.subi(4.5);
    n.subi(4.5);
    assertMatrix(m, n);
    n.dealloc();
  }

  @Test
  public void testSubMatrix() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(20, 40);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(20, 40);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.sub(ma), n.sub(na));
    n.dealloc();
    na.dealloc();
  }

  @Test
  public void testSubMatrixInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(20, 40);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(20, 40);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    m.subi(ma);
    n.subi(na);
    assertMatrix(m, n);
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testSubMatrixFail() {
    DoubleMatrix n = DoubleMatrix.rand(20, 40);
    n.sub(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testSubMatrixInPlaceFail() {
    DoubleMatrix n = DoubleMatrix.rand(20, 40);
    n.subi(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  // == Matrix elementwise multiplication ==

  @Test
  public void testMulScalar() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(30, 10);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.mul(2.3), n.mul(2.3));
    n.dealloc();
  }

  @Test
  public void testMulScalarInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(30, 10);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    m.muli(2.3);
    n.muli(2.3);
    assertMatrix(m, n);
    n.dealloc();
  }

  @Test
  public void testMulMatrix() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(30, 10);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(30, 10);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.mul(ma), n.mul(na));
    n.dealloc();
    na.dealloc();
  }

  @Test
  public void testMulMatrixInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(30, 10);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(30, 10);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    m.muli(ma);
    n.muli(na);
    assertMatrix(m, n);
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testMulMatrixFail() {
    DoubleMatrix n = DoubleMatrix.rand(30, 10);
    n.mul(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testMulMatrixInPlaceFail() {
    DoubleMatrix n = DoubleMatrix.rand(30, 10);
    n.muli(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  // == Matrix elementwise division ==

  @Test
  public void testDivScalar() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(14, 14);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.div(0.34), n.div(0.34));
    n.dealloc();
  }

  @Test
  public void testDivScalarInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(14, 14);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    m.div(0.34);
    n.div(0.34);
    assertMatrix(m, n);
    n.dealloc();
  }

  @Test
  public void testDivMatrix() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(14, 14);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(14, 14);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.div(ma), n.div(na));
    n.dealloc();
    na.dealloc();
  }

  @Test
  public void testDivMatrixInPlace() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(14, 14);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(14, 14);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    m.divi(ma);
    n.divi(na);
    assertMatrix(m, n);
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testDivMatrixFail() {
    DoubleMatrix n = DoubleMatrix.rand(14, 14);
    n.div(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testDivMatrixInPlaceFail() {
    DoubleMatrix n = DoubleMatrix.rand(14, 14);
    n.divi(DoubleMatrix.rand(2, 3));
    n.dealloc();
  }

  // == Column stats ==

  @Test
  public void testColumnSums() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(34, 20);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.columnSums(), n.columnSums());
    n.dealloc();
  }

  @Test
  public void testColumnMins() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(34, 20);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.columnMins(), n.columnMins());
    n.dealloc();
  }

  @Test
  public void testColumnMaxs() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(34, 20);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.columnMaxs(), n.columnMaxs());
    n.dealloc();
  }

  @Test
  public void testColumnMeans() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(34, 20);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.columnMeans(), n.columnMeans());
    n.dealloc();
  }

  // == Row stats ==

  @Test
  public void testRowSums() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(23, 17);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.rowSums(), n.rowSums());
    n.dealloc();
  }

  @Test
  public void testRowMins() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(23, 17);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.rowMins(), n.rowMins());
    n.dealloc();
  }

  @Test
  public void testRowMaxs() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(23, 17);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.rowMaxs(), n.rowMaxs());
    n.dealloc();
  }

  @Test
  public void testRowMeans() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(23, 17);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.rowMeans(), n.rowMeans());
    n.dealloc();
  }

  // == Min, max, sum, norm1, norm2 ==

  @Test
  public void testMinMaxSumNormRand() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(56, 23);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertEquals(m.min(), n.min(), EPS);
    assertEquals(m.max(), n.max(), EPS);
    assertEquals(m.sum(), n.sum(), EPS);
    assertEquals(m.norm1(), n.norm1(), EPS);
    assertEquals(m.norm2(), n.norm2(), EPS);
    n.dealloc();
  }

  @Test
  public void testMinMaxSumNormOnes() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.ones(56, 23);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertEquals(m.min(), n.min(), EPS);
    assertEquals(m.max(), n.max(), EPS);
    assertEquals(m.sum(), n.sum(), EPS);
    assertEquals(m.norm1(), n.norm1(), EPS);
    assertEquals(m.norm2(), n.norm2(), EPS);
    n.dealloc();
  }

  // == Transpose ==

  @Test
  public void testTranspose() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(35, 25);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.transpose(), n.transpose());
    assertMatrix(m, n.transpose().transpose());
    n.dealloc();
  }

  // == Diagonal ==

  @Test
  public void testDiag() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(35, 35);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(m.diag(), n.diag());
    n.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testDiagFail() {
    DoubleMatrix n = DoubleMatrix.rand(10, 15);
    n.diag();
  }
}