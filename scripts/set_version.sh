#!/bin/bash
# 
# Description:
#
#   Script to help set package version across 
#
# Usage: 
#   $ ./set_version.sh 0.1.0
# 

# Replace worspace specific dependencies and versions in lockstep
sed -i.bak "s/version = (0-9).(0-9).(0.9)/version = \"$1\"/" ./rastro/Cargo.toml
sed -i.bak "s/version = (0-9).(0-9).(0.9)$/version = \"$1\"/" ./rastro_python/Cargo.toml
sed -i.bak "s/version = \"(0-9).(0-9).(0.9)\" }$/version = \"\^$1\" \}/" ./rastro_python/Cargo.toml

# Remove tempoarary files 
rm ./rust_python_lib/Cargo.toml.bak
rm ./rust_python_module/Cargo.toml.bak