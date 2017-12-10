#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

JAVAH_BIN="$(which javah)"
if [[ -z "$JAVAH_BIN" ]]; then
  # look up JAVA_HOME, and abort if not found
  if [[ -z "$JAVA_HOME" ]]; then
    echo "Error: failed to locate java binaries, please set JAVA_HOME env or explicitly make 'javah' available"
    exit 1
  fi
  JAVAH_BIN="$JAVA_HOME/bin/javah"
fi

if [[ ! -d "$ROOT_DIR/java/target/scala-2.11/classes" ]]; then
  echo "Error: target directory with Java classes does not exit: run 'sbt compile' or 'sbt package' first"
  exit 1
fi

# space separated list of classes
COMPILED_CLASSES="com.github.sadikovi.rustjblas.DoubleMatrix"
CPP_OUTPUT="$ROOT_DIR/cpp"

cd $ROOT_DIR/java/target/scala-2.11/classes
$JAVAH_BIN -d $CPP_OUTPUT $COMPILED_CLASSES
