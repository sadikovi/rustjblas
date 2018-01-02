ROOT_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
JAVA_DIR=$(ROOT_DIR)/java
CPP_DIR=$(ROOT_DIR)/cpp
RUST_DIR=$(ROOT_DIR)/rust
TARGET_DIR=$(ROOT_DIR)/target

# list of classes to be compiled for jni
JNI_CLASSES="com.github.sadikovi.rustjblas.DoubleMatrix"

# java benchmark class
JAVA_BENCH_CLASS="com.github.sadikovi.rustjblas.MatrixBench"

# Rust compile flags
# Consider changing it for performance tuning, e.g. "-C target-cpu=haswell"
# -C lto
# -C inline-threshold=300
# -C target-feature=+avx,+avx2,+sse2,+sse3,+sse4.1,+sse4.2,+sse4a,+ssse3
RUSTFLAGS=-C target-cpu=native
export RUSTFLAGS

.PHONY: all,
	clean_java, clean_rust, clean,
	build_java, build_rust, build,
	release_java, release_rust, release,
	test_java, test_rust, test,
	bench_java, bench_rust, bench,
	jni

all: release

# == clean ==

clean_java:
	cd $(JAVA_DIR) && sbt clean

clean_rust:
	cd $(RUST_DIR) && cargo clean

clean: clean_java clean_rust
	rm -rf $(TARGET_DIR)

# == build ==

build_java:
	cd $(JAVA_DIR) && sbt package

build_rust:
	cd $(RUST_DIR) && cargo build --verbose

build: build_java jni build_rust
	$(ROOT_DIR)/bin/make_lib.sh

# == release ==

release_java: build_java

release_rust:
	cd $(RUST_DIR) && cargo build --release

release: release_java jni release_rust
	$(ROOT_DIR)/bin/make_lib.sh --release

# == test ==

test_java:
	cd $(JAVA_DIR) && SBT_OPTS="-Djava.library.path=$(TARGET_DIR)" sbt test

test_rust:
	cd $(RUST_DIR) && cargo test

test: test_java test_rust

# == jni ==

jni:
	# generate files for jni
	$(ROOT_DIR)/bin/javah -cp $(JAVA_DIR)/target/scala-2.11/classes -d $(CPP_DIR) $(JNI_CLASSES)

# == bench ==

bench_java:
	cd $(JAVA_DIR) && SBT_OPTS="-Xmx2g -Djava.library.path=$(TARGET_DIR)" sbt "test:runMain $(JAVA_BENCH_CLASS)"

bench_rust:
	cd $(RUST_DIR) && cargo bench

bench: bench_java bench_rust
