# rustjblas

Library for using JBLAS `DoubleMatrix` with off-heap allocation.

Native implementation is loaded from Rust code through JNI and use in Java. As mentioned, all
operations are performed off-heap, including matrix allocation.

## Build instructions
Required:
- JDK 7+
- Rust 1.19+
- sbt, gcc, cargo

Clone repository and cd to the project directory
```
git clone https://github.com/sadikovi/rustjblas.git
cd rustjblas
```

Compile Java classes from projectDir/java
```
cd java
sbt package
```
This compiles classes and creates jar that we will use later

Compile rust shared library from projectDir/rust
```
cd rust
cargo build --release
```

Compile C++ source files, assuming that JNI header has been generated and unchanged (default).
See **bin** folder in project directory.
```
./bin/cpp_compile.sh
```
> `bin` folder also contains script to generate .h files for native methods in Java classes.
> Use this when adding new methods to the class, otherwise, no JNI generation is necessary.

At this point you will have 2 libraries `librsjblas.so` and `libcjblas.so` (or .dylib on OSX) and
jar file. You can run code in scala shell (if available) or run Main class.

## Run code
Run scala shell with following options (use DYLD_LIBRARY_PATH on OS X):
```
LD_LIBRARY_PATH=rust/target/release JAVA_OPTS="-Djava.library.path=cpp/target" \
scala -cp java/target/scala-2.11/rustjblas_2.11-0.1.0-SNAPSHOT.jar
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

Or you can run java main class that performs example init and method calls (use DYLD_LIBRARY_PATH on OS X):
```
LD_LIBRARY_PATH=rust/target/release \
java -Djava.library.path=cpp/target -cp java/target/scala-2.11/rustjblas_2.11-0.1.0-SNAPSHOT.jar com.github.sadikovi.Main
```
