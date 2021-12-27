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
  # Documentation dependencies
  pip install -U mkdocs-material

  # Figure dependencies

}

function build_docs {
  # Compile figures
  echo "Rastro Directory $RASTRO_DIR"

  # Compile documents
#  mkdocs build
}

function serve_docs {
  # Change to English directory
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
    *)
        build_docs
        serve_docs
esac