#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

COMPILED_FILES="$ROOT_DIR/cpp/com_github_sadikovi_DoubleMatrix.cpp"

TARGET_DIR="$ROOT_DIR/target/cpp"
LIB_NAME="librustjblas.dylib"

# clean up target dir
if [[ -d "$TARGET_DIR" ]]; then
  rm -r $TARGET_DIR
fi
mkdir -p "$TARGET_DIR"

cd $TARGET_DIR
gcc -Wall -c $COMPILED_FILES \
  -I/usr/local/java/jdk1.7.0_80/include \
  -I/usr/local/java/jdk1.7.0_80/include/linux &&
clang -o $LIB_NAME $TARGET_DIR/*.o -dynamiclib -L$ROOT_DIR/target/debug -lrustjblas
