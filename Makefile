ROOT_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
JAVA_DIR=$(ROOT_DIR)/java
RUST_DIR=$(ROOT_DIR)/rust
CPP_DIR=$(ROOT_DIR)/cpp

# target dir with final artifacts
TARGET_DIR=$(ROOT_DIR)/target

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
	rm -rf $(TARGET_DIR)

# == test ==

test_java:
	# run java tests
	cd $(JAVA_DIR) && SBT_OPTS="-Djava.library.path=$(TARGET_DIR)" sbt test

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
	$(ROOT_DIR)/bin/cpp_compile.sh

build: build_java jni build_rust build_cpp
	# copy artifacts into target folder
	mkdir -p $(TARGET_DIR) && cp $(JAVA_DIR)/target/scala-2.11/*.jar $(TARGET_DIR) && cp $(CPP_DIR)/target/* $(TARGET_DIR)

# == jni ==

jni:
	# generate files for jni
	$(ROOT_DIR)/bin/javah -cp $(JAVA_DIR)/target/scala-2.11/classes -d $(CPP_DIR) $(JNI_CLASSES)
