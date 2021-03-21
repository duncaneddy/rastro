| Pipeline | Cargo | PyPi | Documentation |
| :-----:  | :--:  | :--: | :-----------: |
| [![Pipeline](https://github.com/duncaneddy/rastro/actions/workflows/test.yml/badge.svg)](https://github.com/duncaneddy/rastro/actions/workflows/test.yml) |  |  |  |
 <!-- | [![Pipeline](https://github.com/duncaneddy/rastro/actions/workflows/test.yml/badge.svg)](https://github.com/duncaneddy/rastro/actions/workflows/test.yml) | -- | [![PyPI version](https://badge.fury.io/py/rastro.svg)](https://badge.fury.io/py/rastro) | [![](https://img.shields.io/badge/docs-latest-blue.svg)](https://duncaneddy.github.io/rastro/) | --><!-- -->

# rastro
rastro (Rust + astro = rastro) is an easy-to-use astrodynamics library written
in Rust for performance, parallelism, and memory safety with co-released Python
bindings, for easy integration and use in the python ecosystem.

Astrodynamics is the math and language for studying, understanding, and representing
spacecraft and satellites. The underlying concepts have been known for a long
time (Thank you Newton, Gauss, Kepler, and more). However, current high-fidelity 
astrodynamics software generally runs into the following pitfalls:
1. Expensive, commercially licensed and closed-source software 
2. High-fidelity heritage libraries making integration with modern tools challenging
3. Out-dated APIs leading to extremely steep learning curves

This frequently means researchers and engineers end up reimplementing
common astrodynamics functions and extremely common occurance. Frequent reimplementation
of common code is both error prone and something people should have to spend their
time on to get started in astrodynamics (unless they want to!). This project 
seeks to address this challenge by providing an easy to use library, built in Rust
for performance with bindings to Python for wide interoperability.

This work builds on experience of past projects in building astrodynamics software:
- [brahe](https://github.com/duncaneddy/brahe) A pure-python astrodynamics library
- [SatelliteDynamics.jl](https://github.com/sisl/SatelliteDynamics.jl) A Julia astrodynamics library

### Usage

If you're using rastro for school, research, or commercial endeavour, I'd love 
to know about it! Tweet me [@duncaneddy](https://twitter.com/DuncanEddy)

<!-- ## Documentation

The documentation for the package can be found here: <https://duncaneddy.github.io/rastro/> -->

## Installation

rastro can be used as either a standalone rust library or a python library.

### Python

This package is distributed from PyPi, and can be installed simply with:

```bash
pip3 install rastro
```

### Rust


## License

The rastro package is licensed and distributed under the MIT License to encourage
usage and to make it easy to integrate with other tools.

The only thing asked is that if you do use the package in your work, or appreciate
the project, either send a message or star the project. Knowing that the project
is being actively used is a large motivator for continued development.