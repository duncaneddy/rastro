#!/bin/bash
# 
# Description:
#
#   Script to help set package version across 
#
# Usage: 
#   $ ./set_version.sh 0.1.0 0.1.1
# 

# Replace worspace specific dependencies and versions in lockstep
sed -i.bak "s/version = \"$1\"/version = \"$2\"/" ./rastro/Cargo.toml
sed -i.bak "s/version = \"$1\"$/version = \"$2\"/" ./rastro_python/Cargo.toml
sed -i.bak "s/version = \"$1\" }$/version = \"$2\" \}/" ./rastro_python/Cargo.toml
sed -i.bak "s/version=\"$1\",/version=\"$2\",/" ./rastro_python/setup.py

# Remove tempoarary files 
rm ./rastro/Cargo.toml.bak
rm ./rastro_python/Cargo.toml.bak
rm ./rastro_python/setup.py.bak