#!/bin/bash
#
# Description:
#
#   Script to build documents for the project
#
# Usage:
#   $ ./make_docs.sh
#


function build_setup {
  pip install -U mkdocs-material
}

function build_docs {
    mkdocs build
}

function serve_docs {
    mkdocs serve
}

case ${1:-all} in
    setup)
        build_setup
        ;;
    build)
        build_docs
        ;;
    serve)
            serve_docs
            ;;
    *)
        build_setup
        serve_docs
esac