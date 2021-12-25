#!/bin/bash
# 
# Description:
#
#   Run test suites for both the rust and python packages of these components
#
# Usage: 
#   $ ./run_tests.sh

function test_rust {
  cargo test
}

function test_python {
    cd rastro_python || exit
    pip3 install -r requirements-dev.txt
    pip3 install -e .
    pytest
    cd ..
}

case ${1:-all} in
    rust)
        test_rust
        ;;
    python)
        test_python
        ;;
    *)
        test_rust
        test_python
esac
