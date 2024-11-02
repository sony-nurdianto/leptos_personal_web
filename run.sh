#!/bin/bash

case "$1" in
start)
  echo "Running start commands..."
  stylance ./apps --output-dir . &
  cargo leptos watch
  ;;
build)
  echo "Running build commands..."
  cargo build --release
  ;;
test)
  echo "Running tests..."
  cargo test
  ;;
*)
  echo "Usage: $0 {start|build|test}"
  exit 1
  ;;
esac
