#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

RELEASE_MODE=""
# Command-line options
for i in "$@"; do
  case $i in
    --release)
      RELEASE_MODE=1
      echo "* Release mode enabled"
    shift ;;
  esac
done

# main directories for the script
JAVA_DIR="$ROOT_DIR/java"
CPP_DIR="$ROOT_DIR/cpp"
RUST_DIR="$ROOT_DIR/rust"
TARGET_DIR="$ROOT_DIR/target"
# output options
SHARED_LIB="librustjblas"

if [[ -z "$JAVA_HOME" ]]; then
  echo "Error: cannot find JAVA_HOME"
  exit 1
fi

# clean up and recreate directory
rm -rf "$TARGET_DIR" && mkdir $TARGET_DIR

# copy artifacts
echo "Copy java jar into $TARGET_DIR"
cp $JAVA_DIR/target/scala-2.11/rustjblas*.jar $TARGET_DIR

echo "Copy static libs into $TARGET_DIR"
RUST_TARGET_DIR="$RUST_DIR/target/debug"
if [[ -n "$RELEASE_MODE" ]]; then
  RUST_TARGET_DIR="$RUST_DIR/target/release"
fi

cp $RUST_TARGET_DIR/libwrapper.a $TARGET_DIR
for f in $(find $RUST_TARGET_DIR/build -type f -name 'libopenblas*.a'); do
  filename="${f##*/}"
  echo "Copy $filename as libopenblas.a into $TARGET_DIR"
  cp $f $TARGET_DIR/libopenblas.a
done

# build shared library

# find gcc libgfortran.a
GFORTRAN_PATH="/usr/lib"
for f in $(find /usr -type f -name 'libgfortran.a' 2>&1); do
  if [[ -f $f ]]; then
    GFORTRAN_PATH=$(dirname "$f")
    echo "Found gfortran path as $GFORTRAN_PATH"
    break
  fi
done

# build for the target platform
cd $TARGET_DIR
if [ "$(uname)" == "Darwin" ]; then
  g++ -dynamiclib -Wall -lresolv -o $SHARED_LIB.dylib \
    -I$JAVA_HOME/include \
    -I$JAVA_HOME/include/darwin \
    -L$TARGET_DIR -L$GFORTRAN_PATH -lwrapper -lopenblas -lgfortran $CPP_DIR/jblas_format.cpp
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  # check if execstack is installed
  if [[ -z "$(which execstack)" ]]; then
    echo "Error: 'execstack' is not installed, e.g. run 'apt-get install execstack' to fix this"
    exit 1
  fi

  g++ -Wall -fPIC -c $CPP_DIR/jblas_format.cpp \
    -I$JAVA_HOME/include -I$JAVA_HOME/include/linux && \
  g++ -Wall -shared -o $SHARED_LIB.so *.o \
    -L$TARGET_DIR -L$GFORTRAN_PATH -lwrapper -lopenblas -lgfortran && \
  execstack -c $SHARED_LIB.so # also apply execstack in linux
else
  echo "Unsupported platform $(uname)"
  exit 1
fi

# remove static libs and object files
rm -f $TARGET_DIR/*.a && rm -f $TARGET_DIR/*.o

echo "Ok, build artifacts are in $TARGET_DIR"
