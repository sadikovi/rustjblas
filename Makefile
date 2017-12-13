JAVA_DIR=java
RUST_DIR=rust
CPP_DIR=cpp

# list of classes to be compiled for jni
JNI_CLASSES="com.github.sadikovi.rustjblas.DoubleMatrix"

.PHONY: all,
	clean_java, clean_rust, clean_cpp, clean,
	test_java, test_rust, test,
	build_java, build_rust, build,
	jni

all: build

# == clean ==

clean_java:
	# clean java artifacts
	cd $(JAVA_DIR) && sbt clean

clean_rust:
	# clean rust artifacts
	cd $(RUST_DIR) && cargo clean

clean_cpp:
	# clean cpp and lib artifacts
	cd $(CPP_DIR) && rm -rf target

clean: clean_java clean_rust clean_cpp
	rm -rf target

# == test ==

test_java:
	# run java tests
	cd $(JAVA_DIR) && sbt test

test_rust:
	# run rust tests
	cd $(RUST_DIR) && cargo test

test: test_java test_rust

# == build ==

build_java:
	# compile java classes and generate package
	cd $(JAVA_DIR) && sbt package

build_rust:
	# compile rust code and generate library
	cd $(RUST_DIR) && cargo build

build_cpp:
	# compile cpp shared library
	bin/cpp_compile.sh

build: build_java jni build_rust build_cpp
	# copy artifacts into target folder
	mkdir -p target && cp $(JAVA_DIR)/target/scala-2.11/*.jar target && cp $(CPP_DIR)/target/* target

# == jni ==

jni:
	# generate files for jni
	bin/javah -cp $(JAVA_DIR)/target/scala-2.11/classes -d $(CPP_DIR) $(JNI_CLASSES)
