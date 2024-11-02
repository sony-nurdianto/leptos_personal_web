#!/bin/bash

case "$1" in
start)
  echo "Running start commands..."
  stylance ./apps --output-dir . &
  cargo leptos watch
  ;;
dev)
  echo "Running dev commands..."
  cargo leptos watch &
  stylance --watch --output-dir . ./apps
  ;;
build)
  echo "Running build commands..."
  cargo build --release
  ;;
test)
  echo "Running tests..."
  cargo test
  ;;
format_app)
  echo "Formating Apps"
  leptosfmt ./apps/src/
  ;;
*)
  echo "Usage: $0 {start|build|test}"
  exit 1
  ;;
esac
