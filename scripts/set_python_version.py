#!/usr/bin/env python
import toml

## Extact Version invormation

# Parse cargo file from main package
cargo = toml.load('./rastro/Cargo.toml')

# Extact version string
vstr = cargo['package']['version']

## Update Python Package Cargo.toml
python_cargo = toml.load('./rastro_python/Cargo.toml')

# Set Package Version
python_cargo['package']['version'] = vstr

# Set rastro crate version dependency
python_cargo['dependencies']['rastro'] = vstr

toml.dump(python_cargo, open('./rastro_python/Cargo.toml', 'w'))

## Update Setup.py Version

# Read in data from input file
fin  = open("./rastro_python/setup.py", "rt")
data = fin.read()
fin.close()

data = data.replace('0.0.0', vstr)

fout = open("./rastro_python/setup.py", "wt")
fout.write(data)
fout.close()