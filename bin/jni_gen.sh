#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

JAVAH_BIN="$(which javah)"
if [[ -z "$JAVAH_BIN" ]]; then
  JAVA_BIN="$(which java)"
  if [[ -z "$JAVA_BIN" ]]; then
    echo "Error: failed to locate java binaries"
    exit 1
  fi
  JAVAH_BIN="${dirname $JAVA_BIN}/javah"
fi

if [[ ! -d "$ROOT_DIR/target/scala-2.11/classes" ]]; then
  echo "Error: target directory with Java classes does not exit: run 'sbt compile' first"
  exit 1
fi

# create stubs
COMPILED_CLASSES="com.github.sadikovi.DoubleMatrix"
CPP_OUTPUT="$ROOT_DIR/cpp"

cd $ROOT_DIR/target/scala-2.11/classes
$JAVAH_BIN -d $CPP_OUTPUT $COMPILED_CLASSES
