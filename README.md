# rustjblas

Library to load Rust code through JNI and use in Java. Matrix is allocated off-heap.

## Build and run
First, compile rust static library
```
cargo build
```

Then compile C++ source files, assuming that JNI header has been generated and unchanged (default)
```
./bin/cpp_compile
```

Package java source files
```
sbt package
```

Now you can run scala shell with following options:
```
JAVA_OPTS="-Djava.library.path=target/cpp" scala -cp target/scala-2.11/rustjblas_2.11-0.1.0-SNAPSHOT.jar
```

... and try creating instances in shell:
```scala
val t = com.github.sadikovi.DoubleMatrix.anew(2, 2, Array(1.0, 2.0, 3.0, 4.0))
t.show()
t.rows
t.cols

val t = com.github.sadikovi.DoubleMatrix.rand(20, 10)
t.show()
```

Or you can run java main class that performs example init and method calls:
```
java -Djava.library.path=target/cpp -cp target/scala-2.11/rustjblas_2.11-0.1.0-SNAPSHOT.jar com.github.sadikovi.Main
```
