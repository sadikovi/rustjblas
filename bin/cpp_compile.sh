#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

COMPILED_FILES="$ROOT_DIR/cpp/com_github_sadikovi_rustjblas_DoubleMatrix.cpp"

TARGET_DIR="$ROOT_DIR/cpp/target"

LIB_NAME="libcjblas"

if [ "$(uname)" == "Darwin" ]; then
  LIB_NAME="$LIB_NAME.dylib"
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  LIB_NAME="$LIB_NAME.so"
else
  echo "Error: unsupported os"
  exit 1
fi

if [[ -z "$JAVA_HOME" ]]; then
  echo "Error: cannot find JAVA_HOME"
  exit 1
fi

# clean up target dir
if [[ -d "$TARGET_DIR" ]]; then
  rm -r $TARGET_DIR
fi
mkdir -p "$TARGET_DIR"

# check that rust library has been compiled
if [[ ! -d "$ROOT_DIR/rust/target/release" ]]; then
  echo "Error: cannot find rust library, build lib using 'cargo build --release' from 'rust' dir"
  exit 1
fi

cd $TARGET_DIR
gcc -Wall -shared -o $LIB_NAME \
  -I$JAVA_HOME/include \
  -I$JAVA_HOME/include/linux \
  -L$ROOT_DIR/rust/target/release -lrsjblas $COMPILED_FILES &&
echo "Compiled $TARGET_DIR/$LIB_NAME"
