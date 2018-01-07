# rustjblas

[![Build Status](https://travis-ci.org/sadikovi/rustjblas.svg?branch=master)](https://travis-ci.org/sadikovi/rustjblas)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Library for using JBLAS `DoubleMatrix` (https://github.com/mikiobraun/jblas) with off-heap
allocation.

Native implementation is loaded from Rust code through JNI and use in Java. As mentioned, all
operations are performed off-heap, including matrix allocation.

## Build instructions
Required:
- JDK 7+
- Rust 1.19+
- sbt, cargo, g++, gfortran

Clone repository and cd to the project directory
```
git clone https://github.com/sadikovi/rustjblas.git
cd rustjblas
```

Run `make` from the project directory to build all (Java, C++/Rust) artifacts into library. After
build is finished, all necessary artifacts will be copied into `target` folder in the project
directory, and should contain `.so` (`.dylib` on OSX) library and jar.

## Run example
Once project is built, run scala-shell with following options from the project directory to see
the code in action (`target` folder should contain both native library and jar):
```
JAVA_OPTS="-Djava.library.path=target" scala -cp target/rustjblas_2.11-0.1.0-SNAPSHOT.jar
```

scala-shell sample commands:
```scala
import com.github.sadikovi.rustjblas.DoubleMatrix
val m = DoubleMatrix.rand(5, 6)
m.show()
m.rows
m.cols
m.add(4.5).show()
m.dealloc
```

## Development

### Build in dev mode
Run `make build` to build project in dev mode (code less optimised, but faster compilation and
verbose output). You could also build subprojects separately by running either `make build_java` or
`make build_rust`. Note that those commands just invoke `sbt` and `cargo`, so you could those to
build the subprojects as well.

### Build in release mode
This is default behaviour of running `make` as an shortcut for `make release`. Similar to `make build`,
this can be run separately for each subproject.

### Run tests
Run `make test` from the project directory to run all tests. Subproject tests, e.g. Java, can be run
either with `make test_java` or using `sbt` from a subproject folder; similar for Rust. Note that it
is required to build project before that to prepare libs.

### Run clean
Run `make clean` to remove temporary files and generated artifacts.

### Compile JNI
Run `make jni` (runs as part of build command) to generate fresh JNI files.

### Run benchmarks
Run `make bench` to run benchmarks (requires nightly Rust), or run specific benchmarks, e.g.
`make bench_rust` or `make bench_java`. It is required to build project before that to prepare libs.

Current benchmark numbers:
```
Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix allocations:                                Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Allocate rand matrix (jblas) n = 2000                     91 /   92       1.0X
Allocate rand matrix (rustjblas), n = 2000                18 /   19       5.0X
Allocate identity matrix (jblas) n = 2000                  3 /    4      29.0X
Allocate identity matrix (rustjblas), n = 2000            18 /   21       5.0X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix transformations:                            Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Matrix transpose (jblas), n = 2000                        16 /   17       1.0X
Matrix transpose (rustjblas), n = 2000                    24 /   25       0.7X
Matrix absolute (jblas), n = 2000                         11 /   12       1.4X
Matrix absolute (rustjblas), n = 2000                      6 /   20       2.9X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Scalar Elementwise operations:                     Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Scalar addition (jblas), n = 2000                          8 /    9       1.0X
Scalar addition (rustjblas), n = 2000                      9 /   12       0.9X
Scalar subtraction (jblas), n = 2000                       8 /    9       1.0X
Scalar subtraction (rustjblas), n = 2000                   9 /   12       0.9X
Scalar multiplication (jblas), n = 2000                    8 /    9       1.0X
Scalar multiplication (rustjblas), n = 2000                9 /   13       0.9X
Scalar division (jblas), n = 2000                         12 /   12       0.7X
Scalar division (rustjblas), n = 2000                      9 /   13       0.9X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix elementwise operations:                     Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Matrix addition (jblas), n = 2000                         10 /   11       1.0X
Matrix addition (rustjblas), n = 2000                     10 /   12       1.0X
Matrix subtraction (jblas), n = 2000                      13 /   14       0.8X
Matrix subtraction (rustjblas), n = 2000                  10 /   12       1.0X
Matrix multiplication (jblas), n = 2000                   10 /   11       1.0X
Matrix multiplication (rustjblas), n = 2000               11 /   13       1.0X
Matrix division (jblas), n = 2000                         11 /   12       0.9X
Matrix division (rustjblas), n = 2000                     23 /   24       0.5X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix-matrix operations:                          Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Matrix multiplication (jblas), n = 2000                 1329 / 1331       1.0X
Matrix multiplication (rustjblas), n = 2000              129 /  134      10.3X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix SVD operations:                             Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Full SVD (jblas), n = 1000                              4582 / 4596       1.0X
Full SVD (rustjblas), n = 1000                           398 /  403      11.5X
SVD k=5 (rustjblas), n = 1000                            172 /  173      26.7X
SVD k=20 (rustjblas), n = 1000                           188 /  189      24.4X
Singular values (jblas), n = 1000                        442 /  444      10.4X
Singular values (rustjblas), n = 1000                    176 /  180      26.0X
```
