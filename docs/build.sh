#!/bin/bash
#
# Description:
#
#   Script to build documents for the project
#
# Usage:
#   $ ./make_docs.sh
#

# Directory of build script
SCRIPT_DIR="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )" || return
export RASTRO_FIGURE_OUTPUT_DIR="$SCRIPT_DIR/en/docs/figures"

function install_dependencies {
  echo "Installing dependencies"
  pip install -U -r "$SCRIPT_DIR/requirements.txt"

  cargo install rust-script
}

function build_figures {
  # Change to figures directory
  if [[ $(pwd) != "$SCRIPT_DIR/figures" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/figures"
      cd "$SCRIPT_DIR/figures" || exit 1
  fi

  # Ensure directory exists
  mkdir -p "$RASTRO_FIGURE_OUTPUT_DIR"

  for i in *.py; do
      (echo "Building figure $i" && python "$i") || (echo "Error building
      figure $i. Exiting without completing" && exit 1)
  done

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1

  # Clean up environment variable
  unset RASTRO_FIGURE_OUTPUT_DIR
}

# test_examples attempts to execute every file in the ./docs/examples directory
# This folder contains all of the code examples used as part of the
# documentation.
function test_examples {
  # Change to figures directory
  if [[ $(pwd) != "$SCRIPT_DIR/examples" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/examples"
      cd "$SCRIPT_DIR/examples" || exit 1
  fi

  # Test Rust code examples
  echo "Testing rust code examples"
  for i in *.rs; do
      echo "Testing example $i"

      # Create temporary file
#      tmpfile=$(mktemp ./test.rs)
      tmpfile=$(mktemp /tmp/rust-script.rs)

      # Add appropriate local dependency to header
      echo "//! \`\`\`cargo" >> $tmpfile
      echo "//! [dependencies]" >> $tmpfile
      echo "//! rastro = {path = \"$SCRIPT_DIR/../rastro\"}" >> $tmpfile
      echo "//! \`\`\`" >> $tmpfile
      cat "$i" >> $tmpfile

      # Build
      echo "$tmpfile"
      (printf %s "Test example $i" && (rust-script "$tmpfile" >> /dev/null) &&
      echo "...done") ||
      (echo "Error building figure $i. Exiting without completing" &&
      rm "$tmpfile" && exit 1)

      rm "$tmpfile"
  done

  # Test Python code examples
  echo "Testing python code examples"
  for i in *.py; do
      (printf %s "Testing example $i" && (python "$i" >> /dev/null) &&
      echo "...done") ||
      (echo "Error testing figure $i. Exiting without completing example tests" && exit 1)
  done
}

function build_docs {
  echo "Beginning build step"

  # Build Figures
  echo "Building figures..."
  build_figures

  # Build Docs
  echo "Building documentation..."
  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || exit 1
  fi

  # Compile documents
  mkdocs build

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1
}

function publish_docs {
  echo "Beginning Publish Step"

  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || exit 1
  fi

  # Compile documents
  mkdocs gh-deploy --force

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1
}

function serve_docs {
  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || exit 1
  fi

  # Serve documents
  mkdocs serve

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1
}

case ${1:-all} in
    install)
        install_dependencies
        ;;
    test)
        test_examples
        ;;
    build)
        build_docs
        ;;
    serve)
        serve_docs
        ;;
    publish)
        publish_docs
        ;;
    *)
        test_examples
        build_docs
        serve_docs
esac