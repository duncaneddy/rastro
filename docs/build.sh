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
export RASTRO_FIGURE_OUTPUT_DIR="$SCRIPT_DIR/en/docs/figures/"

function install_dependencies {
  echo "Install rastro pyhton"
  pip install -U -e ./rastro_python

  echo "Installing example and figure dependencies"
  pip install -U -r "$SCRIPT_DIR/requirements.txt"

  echo "Installing rust-script"
  cargo install rust-script
}

function change_dir {
  target_dir="$SCRIPT_DIR/$1"
  if [[ $(pwd) != "$target_dir" ]]; then
#      echo "Changing directory to: $target_dir"
      cd "$target_dir" || exit 1
  fi
}

function build_figures {
  # Change to figures directory
  change_dir "figures"

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

# Build and execute a standalone rust script
# If second argument is present supress output
function run_rust_script () {
  tmpfile=$(mktemp /tmp/rust-script.rs)

  # Add appropriate local dependency to header
  echo "//! \`\`\`cargo" >> $tmpfile
  echo "//! [dependencies]" >> $tmpfile
  echo "//! rastro = {path = \"$SCRIPT_DIR/../rastro\"}" >> $tmpfile
  echo "//! approx = \"^0.4.0\"" >> $tmpfile
  echo "//! \`\`\`" >> $tmpfile
  cat "$1" >> $tmpfile

  # Execute script
  if [[ "$2" == "quiet" ]]; then
    (printf %s "Test example $1" && (rust-script "$tmpfile" >> /dev/null) &&
    echo "...done") ||
    (echo "Error building figure $1. Exiting without completing" &&
    rm "$tmpfile" && exit 1)
  else
    (echo "Executing: $1" && (rust-script "$tmpfile")) ||
    (echo "Error building figure $1. Exiting without completing" &&
    rm "$tmpfile" && exit 1)
  fi

  rm "$tmpfile"
}

# Build and execute a standalone python script
# If second argument is present supress output
function run_python_script () {
  if [[ "$2" == "quiet" ]]; then
    (printf %s "Testing example $1" && (python "$1" >> /dev/null) &&
    echo "...done") || (echo "Error testing figure $1. Exiting without completing example tests" && exit 1)
  else
    (echo "Execuing: $1" && (python "$1")) ||
    (echo "Error testing figure $1. Exiting without completing example tests" && exit 1)
  fi
}

# test_examples attempts to execute every file in the ./docs/examples directory
# This folder contains all of the code examples used as part of the
# documentation.
function test_examples {
  # Change to examples
  change_dir "examples"

  # Test Rust code examples
  echo "" && echo "=== Testing Rust code examples ===" && echo ""
  for i in *.rs; do
      run_rust_script "$i" "quiet"
  done

  # Test Python code examples
  echo "" && echo "=== Testing Python code examples ===" && echo ""
  for i in *.py; do
      run_python_script "$i" "quiet"
  done
}

function test_example_script {
  filename="${BASH_ARGV[0]}"
  extension="${filename##*.}"

  if [[ "$extension" == "rs" ]]; then
    run_rust_script "$filename"
  elif [[ "$extension" == "py" ]]; then
    run_python_script "$filename"
  else
    echo "Unknown script extension \"$extension\""
    exit 1
  fi
}

function build_docs {
  echo "Beginning build step"

  # Build Figures
  echo "Building figures..."
  build_figures

  # Build Docs
  echo "Building documentation..."
  # Change to English docs directory
  change_dir "en"

  # Compile documents
  mkdocs build

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1
}

function publish_docs {
  echo "Beginning Publish Step"

  # Change to English docs directory
  change_dir "en"

  # Compile documents
  mkdocs gh-deploy --force

  # Return to source director
  cd "$SCRIPT_DIR" || exit 1
}

function serve_docs {
  # Change to English docs directory
  change_dir "en"

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
    example)
        test_example_script
        ;;
    figures)
        build_figures
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