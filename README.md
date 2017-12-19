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
- sbt, cargo, g++, gfortran, libblas3gf, liblapack3gf, execstack (optional)

Clone repository and cd to the project directory
```
git clone https://github.com/sadikovi/rustjblas.git
cd rustjblas
```

Run `make` or `make build` from the project directory to build all (Java, Rust, C++). Or use
separate commands, e.g. `make build_java` or `make build_rust` to build individual subprojects.

Each subproject can also be built from its own folder with `sbt` or `cargo`.

After build is finished, all necessary artifacts will be copied into `target` folder in project
directory, and should contain shared `.so`/`.dylib` library and jar.

## Run sample code
Run scala-shell with following options from the project directory, `target` folder should contain
both shared library and jar:
```
JAVA_OPTS="-Djava.library.path=target" scala -cp target/rustjblas_2.11-0.1.0-SNAPSHOT.jar
```

and try creating matrices in scala-shell:
```scala
import com.github.sadikovi.rustjblas.DoubleMatrix
val t = DoubleMatrix.rand(5, 6)
t.show()
t.rows
t.cols
t.add(4.5).show()
t.dealloc
```

## Development

### Run test
Run `make test` from the project directory to run all tests. Subproject tests, e.g. Java, can be run
either with `make test_java` or using `sbt` from a subproject folder; similar for Rust. Note that it
is required to build project before that to prepare libs.

### Run clean
Run `make clean` to remove temporary files and generated artifacts.

### Compile JNI
Run `make jni` (runs as part of build command) to generate fresh JNI files.

### Run benchmarks
Run `make bench` to run benchmarks (requires nightly), or run specific benchmarks, e.g.
`make bench_rust` or `make bench_java`. It is required to build project before that to prepare libs.

Current benchmark numbers:
```
Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix elementwise operations:                     Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Allocate rand matrix (jblas) n = 2000                     89 /   90       1.0X
Allocate rand matrix (rustjblas), n = 2000                43 /   48       2.0X
Allocate identity matrix (jblas) n = 2000                  3 /    3      28.3X
Allocate identity matrix (rustjblas), n = 2000            25 /   30       3.5X
Matrix addition (jblas), n = 2000                         10 /   11       8.8X
Matrix addition (rustjblas), n = 2000                     27 /   28       3.3X
Matrix subtraction (jblas), n = 2000                      13 /   14       6.8X
Matrix subtraction (rustjblas), n = 2000                  27 /   28       3.3X
Matrix multiplication (jblas), n = 2000                   10 /   11       9.0X
Matrix multiplication (rustjblas), n = 2000               32 /   34       2.8X
Matrix division (jblas), n = 2000                         12 /   12       7.6X
Matrix division (rustjblas), n = 2000                     25 /   38       3.5X

Java HotSpot(TM) 64-Bit Server VM 1.8.0_101-b13 on Linux 3.16.0-70-generic
Intel(R) Core(TM) i7-4700MQ CPU @ 2.40GHz
Matrix-matrix operations:                          Best/Avg Time(ms)   Relative
-------------------------------------------------------------------------------
Matrix multiplication (jblas), n = 2000                 1318 / 1363       1.0X
Matrix multiplication (rustjblas), n = 2000              126 /  129      10.4X
```
