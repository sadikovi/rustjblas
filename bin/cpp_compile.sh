#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

CPP_FILE="$ROOT_DIR/cpp/jblas_interface.cpp"
TARGET_DIR="$ROOT_DIR/cpp/target"
SHARED_LIB="libntvjblas"
RUST_OUTPUT="$ROOT_DIR/rust/target/debug"
RUST_STATIC_LIB="ntvjblas"

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
if [[ ! -d "$ROOT_DIR/rust/target/debug" ]]; then
  echo "Error: cannot find rust library, build lib using 'cargo build --release' from 'rust' dir"
  exit 1
fi

cd $TARGET_DIR
if [ "$(uname)" == "Darwin" ]; then
  g++ -dynamiclib -Wall -lresolv -o $SHARED_LIB.dylib \
    -I$JAVA_HOME/include \
    -I$JAVA_HOME/include/darwin \
    -L$RUST_OUTPUT -l$RUST_STATIC_LIB $CPP_FILE
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  g++ -shared -Wall -lresolv -o $SHARED_LIB.so \
    -I$JAVA_HOME/include \
    -I$JAVA_HOME/include/linux \
    -LRUST_OUTPUT -l$RUST_STATIC_LIB $CPP_FILE
else
  echo "Error: unsupported os"
  exit 1
fi
