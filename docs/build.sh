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

function install_dependencies {
  echo "Installing dependencies"

  # Documentation dependencies
  pip install -U mkdocs-material mkdocstrings

  # Figure dependencies

}

function build_docs {
  echo "Beginning build step"

  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || return
  fi

  # Compile documents
  mkdocs build

  # Return to source director
  cd "$SCRIPT_DIR" || return
}

function publish_docs {
  echo "Beginning Publish Step"

  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || return
  fi

  # Compile documents
  mkdocs gh-deploy --force

  # Return to source director
  cd "$SCRIPT_DIR" || return
}

function serve_docs {
  # Change to English docs directory
  if [[ $(pwd) != "$SCRIPT_DIR/en" ]]; then
      echo "Changing directory to: $SCRIPT_DIR/en"
      cd "$SCRIPT_DIR/en" || return
  fi

  # Serve documents
  mkdocs serve

  # Return to source director
  cd "$SCRIPT_DIR" || return
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