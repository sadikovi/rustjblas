#!/bin/bash

bin="`dirname "$0"`"
ROOT_DIR="`cd "$bin/../"; pwd`"

cd $ROOT_DIR/java && sbt clean && \
cd $ROOT_DIR/rust && cargo clean && \
cd $ROOT_DIR/cpp && rm -rf target
