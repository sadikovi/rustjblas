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

import org.jblas.{DoubleMatrix => JDoubleMatrix}

object MatrixBench {
  // matrix size for elementwise operations
  val EW_SIZE = 2000
  // matrix size for matrix-matrix operations
  val MM_SIZE = 2000

  def main(args: Array[String]): Unit = {
    println("\nNOTE: For better performance info, make sure to build library with optimizations\n")

    val a1 = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val b1 = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val m1 = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
    val n1 = DoubleMatrix.rand(EW_SIZE, EW_SIZE)

    val ewBench = new Benchmark("Matrix elementwise operations")
    ewBench.addCase(s"Allocate rand matrix (jblas) n = $EW_SIZE") { iter =>
      JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    }
    ewBench.addCase(s"Allocate rand matrix (rustjblas), n = $EW_SIZE") { iter =>
      DoubleMatrix.rand(EW_SIZE, EW_SIZE)
    }
    ewBench.addCase(s"Allocate identity matrix (jblas) n = $EW_SIZE") { iter =>
      JDoubleMatrix.eye(EW_SIZE)
    }
    ewBench.addCase(s"Allocate identity matrix (rustjblas), n = $EW_SIZE") { iter =>
      DoubleMatrix.eye(EW_SIZE)
    }
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

    ewBench.run
    mmBench.run
  }
}
