#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

# build java code -> rust code -> cpp code
cd $ROOT_DIR/java && sbt package && \
cd $ROOT_DIR/rust && cargo build --release && \
$ROOT_DIR/bin/cpp_compile.sh
