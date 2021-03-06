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

package com.github.sadikovi.rustjblas

import org.jblas.{DoubleMatrix => JDoubleMatrix, MatrixFunctions, Singular}

object MatrixBench {
  // matrix size for matrix allocations
  val AL_SIZE = 2000
  // matrix size for matrix transformations
  val TR_SIZE = 2000
  // matrix size for elementwise operations
  val EW_SIZE = 2000
  // matrix size for matrix-matrix operations
  val MM_SIZE = 2000
  // matrix size for SVD operations
  val SVD_SIZE = 1000

  def main(args: Array[String]): Unit = {
    println("\nNOTE: For better performance info, make sure to build library with optimizations\n")

    val alBench = new Benchmark("Matrix allocations")
    alBench.addCase(s"Allocate rand matrix (jblas) n = $AL_SIZE") { iter => JDoubleMatrix.rand(AL_SIZE, AL_SIZE) }
    alBench.addCase(s"Allocate rand matrix (rustjblas), n = $AL_SIZE") { iter => DoubleMatrix.rand(AL_SIZE, AL_SIZE) }
    alBench.addCase(s"Allocate identity matrix (jblas) n = $AL_SIZE") { iter => JDoubleMatrix.eye(AL_SIZE) }
    alBench.addCase(s"Allocate identity matrix (rustjblas), n = $AL_SIZE") { iter => DoubleMatrix.eye(AL_SIZE) }

    val a0 = JDoubleMatrix.rand(TR_SIZE, TR_SIZE)
    val m0 = DoubleMatrix.rand(TR_SIZE, TR_SIZE)

    val trBench = new Benchmark("Matrix transformations")
    trBench.addCase(s"Matrix transpose (jblas), n = $TR_SIZE") { iter => a0.transpose() }
    trBench.addCase(s"Matrix transpose (rustjblas), n = $TR_SIZE") { iter => m0.transpose() }
    trBench.addCase(s"Matrix absolute (jblas), n = $TR_SIZE") { iter => MatrixFunctions.abs(a0) }
    trBench.addCase(s"Matrix absolute (rustjblas), n = $TR_SIZE") { iter => m0.abs() }
    // trBench.addCase(s"Matrix diagonal (jblas), n = $TR_SIZE") { iter => a0.diag() }
    // trBench.addCase(s"Matrix diagonal (rustjblas), n = $TR_SIZE") { iter => m0.diag() }

    val a1 = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val b1 = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val m1 = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val n1 = DoubleMatrix.rand(EW_SIZE, EW_SIZE)

    val seBench = new Benchmark("Scalar Elementwise operations")
    seBench.addCase(s"Scalar addition (jblas), n = $EW_SIZE") { iter => a1.add(3.1) }
    seBench.addCase(s"Scalar addition (rustjblas), n = $EW_SIZE") { iter => m1.add(3.1) }
    seBench.addCase(s"Scalar subtraction (jblas), n = $EW_SIZE") { iter => a1.sub(3.2) }
    seBench.addCase(s"Scalar subtraction (rustjblas), n = $EW_SIZE") { iter => m1.sub(3.2) }
    seBench.addCase(s"Scalar multiplication (jblas), n = $EW_SIZE") { iter => a1.mul(3.3) }
    seBench.addCase(s"Scalar multiplication (rustjblas), n = $EW_SIZE") { iter => m1.mul(3.3) }
    seBench.addCase(s"Scalar division (jblas), n = $EW_SIZE") { iter => a1.div(3.4) }
    seBench.addCase(s"Scalar division (rustjblas), n = $EW_SIZE") { iter => m1.div(3.4) }

    val ewBench = new Benchmark("Matrix elementwise operations")
    ewBench.addCase(s"Matrix addition (jblas), n = $EW_SIZE") { iter => a1.add(b1) }
    ewBench.addCase(s"Matrix addition (rustjblas), n = $EW_SIZE") { iter => m1.add(n1) }
    ewBench.addCase(s"Matrix subtraction (jblas), n = $EW_SIZE") { iter => a1.sub(b1) }
    ewBench.addCase(s"Matrix subtraction (rustjblas), n = $EW_SIZE") { iter => m1.sub(n1) }
    ewBench.addCase(s"Matrix multiplication (jblas), n = $EW_SIZE") { iter => a1.mul(b1) }
    ewBench.addCase(s"Matrix multiplication (rustjblas), n = $EW_SIZE") { iter => m1.mul(n1) }
    ewBench.addCase(s"Matrix division (jblas), n = $EW_SIZE") { iter => a1.div(b1) }
    ewBench.addCase(s"Matrix division (rustjblas), n = $EW_SIZE") { iter => m1.div(n1) }

    val a2 = JDoubleMatrix.rand(MM_SIZE, MM_SIZE)
    val b2 = JDoubleMatrix.rand(MM_SIZE, MM_SIZE)
    val m2 = DoubleMatrix.rand(MM_SIZE, MM_SIZE)
    val n2 = DoubleMatrix.rand(MM_SIZE, MM_SIZE)

    val mmBench = new Benchmark("Matrix-matrix operations")
    mmBench.addCase(s"Matrix multiplication (jblas), n = $MM_SIZE") { iter => a2.mmul(b2) }
    mmBench.addCase(s"Matrix multiplication (rustjblas), n = $MM_SIZE") { iter => m2.mmul(n2) }

    val a3 = JDoubleMatrix.rand(SVD_SIZE, SVD_SIZE)
    val m3 = DoubleMatrix.rand(SVD_SIZE, SVD_SIZE)

    val svdBench = new Benchmark("Matrix SVD operations")
    svdBench.addCase(s"Full SVD (jblas), n = $SVD_SIZE") { iter => Singular.fullSVD(a3) }
    svdBench.addCase(s"Full SVD (rustjblas), n = $SVD_SIZE") { iter => m3.fullSVD() }
    svdBench.addCase(s"SVD k=5 (rustjblas), n = $SVD_SIZE") { iter => m3.svd(5) }
    svdBench.addCase(s"SVD k=20 (rustjblas), n = $SVD_SIZE") { iter => m3.svd(20) }
    svdBench.addCase(s"Lanczos k=5 (rustjblas), n = $SVD_SIZE") { iter => m3.lansvd(5) }
    svdBench.addCase(s"Lanczos k=20 (rustjblas), n = $SVD_SIZE") { iter => m3.lansvd(20) }
    svdBench.addCase(s"Singular values (jblas), n = $SVD_SIZE") { iter => Singular.SVDValues(a3) }
    svdBench.addCase(s"Singular values (rustjblas), n = $SVD_SIZE") { iter => m3.singularValues() }

    alBench.run
    trBench.run
    seBench.run
    ewBench.run
    mmBench.run
    svdBench.run
  }
}
