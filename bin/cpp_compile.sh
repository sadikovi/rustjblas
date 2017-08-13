#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

COMPILED_FILES="$ROOT_DIR/cpp/com_github_sadikovi_DoubleMatrix.cpp"

TARGET_DIR="$ROOT_DIR/target/cpp"

LIB_NAME="librustjblas"

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

cd $TARGET_DIR
gcc -Wall -c -fPIC $COMPILED_FILES \
  -I$JAVA_HOME/include \
  -I$JAVA_HOME/include/linux &&
gcc -o $LIB_NAME $TARGET_DIR/*.o -shared -L$ROOT_DIR/target/debug -lrustjblas &&
echo "Compiled $TARGET_DIR/$LIB_NAME"
