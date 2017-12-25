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

  @Test
  public void testMatrixMultiply() {
    // square matrix
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(10, 10);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(10, 10);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.mmul(ma), n.mmul(na));
    n.dealloc();
    na.dealloc();

    // non-square matrix
    m = org.jblas.DoubleMatrix.rand(14, 8);
    ma = org.jblas.DoubleMatrix.rand(8, 20);
    n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.mmul(ma), n.mmul(na));
    n.dealloc();
    na.dealloc();

    // row and column vectors into matrix
    m = org.jblas.DoubleMatrix.rand(10, 1);
    ma = org.jblas.DoubleMatrix.rand(1, 10);
    n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.mmul(ma), n.mmul(na));
    n.dealloc();
    na.dealloc();

    // row and column vectors as dot product
    m = org.jblas.DoubleMatrix.rand(1, 15);
    ma = org.jblas.DoubleMatrix.rand(15, 1);
    n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    assertMatrix(m.mmul(ma), n.mmul(na));
    n.dealloc();
    na.dealloc();
  }

  @Test
  public void testMatrixMultiplyInPlace() {
    // square matrix
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(3, 3);
    org.jblas.DoubleMatrix ma = org.jblas.DoubleMatrix.rand(3, 3);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    DoubleMatrix na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    m.mmuli(ma);
    n.mmuli(na);
    assertMatrix(m, n);
    n.dealloc();
    na.dealloc();

    // non-square matrix
    m = org.jblas.DoubleMatrix.rand(14, 8);
    ma = org.jblas.DoubleMatrix.rand(8, 20);
    n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    na = DoubleMatrix.fromArray(ma.rows, ma.columns, ma.toArray());
    n.mmuli(na);
    assertMatrix(m.mmul(ma), n);
    n.dealloc();
    na.dealloc();
  }

  @Test(expected = OperationException.class)
  public void testMatrixMultiplyFail() {
    DoubleMatrix n = DoubleMatrix.rand(5, 7);
    n.mmul(DoubleMatrix.rand(3, 4));
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

  // == Absolute ==

  @Test
  public void testAbs() {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(35, 35);
    m.negi();
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(org.jblas.MatrixFunctions.abs(m), n.abs());
    // check that original matrix is not updated
    assertMatrix(m, n);
    n.dealloc();
  }

  @Test
  public void testAbsPartial() {
    DoubleMatrix m = DoubleMatrix.fromArray(2, 2, new double[]{1.0, -2.0, 3.0, -4.0});
    assertArrayEquals(m.abs().toArray(), new double[]{1.0, 2.0, 3.0, 4.0}, EPS);
    m.dealloc();
  }

  // == Singular value decomposition

  private void testFullSVD(int rows, int cols, double offset) {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(rows, cols);
    m.addi(offset);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    org.jblas.DoubleMatrix[] res1 = org.jblas.Singular.fullSVD(m);
    DoubleMatrix[] res2 = n.fullSVD();
    // check that original matrix is not modified
    assertMatrix(m, n);
    // check svd output
    assertMatrix(res1[0], res2[0]);
    assertMatrix(res1[1], res2[1]);
    assertMatrix(res1[2], res2[2]);
  }

  @Test
  public void testFullSVDRows() {
    testFullSVD(20, 10, 0.0);
  }

  @Test
  public void testFullSVDCols() {
    testFullSVD(10, 20, 0.0);
  }

  @Test
  public void testFullSVDSquare() {
    testFullSVD(20, 20, 0.0);
  }

  @Test
  public void testFullSVDSquareOffset() {
    testFullSVD(14, 14, 12.0);
  }

  @Test
  public void testFullSVDSmall() {
    testFullSVD(2, 2, 9.3);
  }

  private void testSingularValues(int rows, int cols) {
    org.jblas.DoubleMatrix m = org.jblas.DoubleMatrix.rand(rows, cols);
    DoubleMatrix n = DoubleMatrix.fromArray(m.rows, m.columns, m.toArray());
    assertMatrix(org.jblas.Singular.SVDValues(m), n.singularValues());
  }

  @Test
  public void testSingularValuesSquare() {
    testSingularValues(20, 20);
  }

  @Test
  public void testSingularValuesRows() {
    testSingularValues(20, 11);
  }

  @Test
  public void testSingularValuesCols() {
    testSingularValues(11, 20);
  }
}
