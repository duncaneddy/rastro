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
SCRIPT_DIR=$( dirname "$0" )

function install_dependencies {
  echo "Installing dependencies"

  # Documentation dependencies
  pip install -U mkdocs-material

  # Figure dependencies

}

function build_docs {
  echo "Beginning Publish Step"

  # Change to English docs directory
  echo "Changing directory to: $SCRIPT_DIR/en"
  cd "$SCRIPT_DIR/en"

  # Compile documents
  mkdocs build
}

function publish_docs {
  echo "Beginning Publish Step"

  # Change to English docs directory
  echo "Changing directory to: $SCRIPT_DIR/en"
  cd "$SCRIPT_DIR/en"

  # Compile documents
  mkdocs gh-deploy --force
}

function serve_docs {
  # Change to English docs directory
  echo "Changing directory to: $SCRIPT_DIR/en"
  cd "$SCRIPT_DIR/en"

  # Serve documents
  mkdocs serve
}

case ${1:-all} in
    install)
        install_dependencies
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
        build_docs
        serve_docs
esac