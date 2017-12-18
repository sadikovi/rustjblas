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

object JavaBench {
  // matrix size for elementwise operations
  val EW_SIZE = 1000
  // matrix size for matrix-matrix operations
  val MM_SIZE = 400

  def main(args: Array[String]): Unit = {
    println("\nNOTE: For better performance info, make sure to build library with optimizations\n")

    val ewBench = new Benchmark("Matrix elementwise operations")
    ewBench.addCase(s"Allocate rand matrix (jblas) n = $EW_SIZE") { iter =>
      JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
    }
    ewBench.addCase(s"Allocate rand matrix (rustjblas), n = $EW_SIZE") { iter =>
      DoubleMatrix.rand(EW_SIZE, EW_SIZE)
    }
    ewBench.addCase(s"Matrix addition (jblas), n = $EW_SIZE") { iter =>
      val a = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.add(b)
    }
    ewBench.addCase(s"Matrix addition (rustjblas), n = $EW_SIZE") { iter =>
      val a = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.add(b)
    }
    ewBench.addCase(s"Matrix subtraction (jblas), n = $EW_SIZE") { iter =>
      val a = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.sub(b)
    }
    ewBench.addCase(s"Matrix subtraction (rustjblas), n = $EW_SIZE") { iter =>
      val a = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.sub(b)
    }
    ewBench.addCase(s"Matrix multiplication (jblas), n = $EW_SIZE") { iter =>
      val a = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.mul(b)
    }
    ewBench.addCase(s"Matrix multiplication (rustjblas), n = $EW_SIZE") { iter =>
      val a = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.mul(b)
    }
    ewBench.addCase(s"Matrix division (jblas), n = $EW_SIZE") { iter =>
      val a = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = JDoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.div(b)
    }
    ewBench.addCase(s"Matrix division (rustjblas), n = $EW_SIZE") { iter =>
      val a = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      val b = DoubleMatrix.rand(EW_SIZE, EW_SIZE)
      a.div(b)
    }

    val mmBench = new Benchmark("Matrix-matrix operations")
    mmBench.addCase(s"Matrix multiplication (jblas), n = $MM_SIZE") { iter =>
      val a = JDoubleMatrix.rand(MM_SIZE, MM_SIZE)
      val b = JDoubleMatrix.rand(MM_SIZE, MM_SIZE)
      a.mmul(b)
    }
    mmBench.addCase(s"Matrix multiplication (rustjblas), n = $MM_SIZE") { iter =>
      val a = DoubleMatrix.rand(MM_SIZE, MM_SIZE)
      val b = DoubleMatrix.rand(MM_SIZE, MM_SIZE)
      a.mmul(b)
    }

    ewBench.run
    mmBench.run
  }
}
