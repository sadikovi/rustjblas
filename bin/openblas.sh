#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

# file for successful installation
SUCCESS_MARK="_SUCCESS"
# openblas directory settings
OPENBLAS_SRC="openblas-src"
OPENBLAS_PARENT="$ROOT_DIR/rust"
OPENBLAS_DIR="$ROOT_DIR/rust/$OPENBLAS_SRC"

if [[ ! -f "$OPENBLAS_DIR/$SUCCESS_MARK" ]]; then
  echo "Download and build openblas"
  rm -rf $OPENBLAS_DIR && \
  cd $OPENBLAS_PARENT && \
  curl -LOk https://github.com/xianyi/OpenBLAS/archive/v0.2.20.zip && \
  unzip -q v0.2.20.zip && \
  rm v0.2.20.zip && \
  mv OpenBLAS-0.2.20 $OPENBLAS_DIR && \
  echo "Ok, downloaded into $OPENBLAS_DIR" && \
  cd $OPENBLAS_DIR && make libs netlib shared BINARY=64 NO_CBLAS=1 NO_LAPACKE=1 &>make-output.log && \
  touch "$OPENBLAS_DIR/$SUCCESS_MARK" &&
  echo "Ok, installed into $OPENBLAS_DIR" || \
  echo "Failed to install"
else
  echo "Already built openblas, remove $OPENBLAS_DIR or $SUCCESS_MARK to trigger rebuild"
fi

ls -lh $OPENBLAS_DIR
