[![Tests](https://github.com/duncaneddy/rastro/actions/workflows/test.yml/badge.svg)](https://github.com/duncaneddy/rastro/actions/workflows/test.yml) 
[![crates.io](https://img.shields.io/crates/v/rastro.svg)](https://crates.io/crates/rastro)
[![PyPI](https://badge.fury.io/py/rastro.svg)](https://badge.fury.io/py/rastro)
[![](https://img.shields.io/badge/docs-latest-blue.svg)](https://duncaneddy.github.io/rastro/)

----

Documentation: https://duncaneddy.github.io/rastro

Rust Library Reference: 

Source Code: https://github.com/duncaneddy/rastro

----

# rastro
rastro is a modern, fast (high-performance), space dynamics library for research and engineering applications.

The key features are:
- **Fast**: Very high performance, on par with C++ libraries, thanks to core library being written in rust.
- **Fast to code**: Rastro provides a Python 3.6+ wrapper that is auto-generated from the core rust libraries. Making it easy to use rastro without recompiling.
- **Intuitive**: API designed to be composable, making it easy to build on core concepts.
- **Easy**: Designed to be easy to use and learn. Less time reading docs, more time building.

rastro gets it's name from the combination of Rust and astrodynamics (Rust + astro = rastro). Space dynamics is an expansive field that covers multiple domains of mathematics, astrodynamics, attitude dynamics, and systems engineering. While the underlying concepts have been studied for a long time (Kepler, Newton, Gauss, and many more), there are few modern software libraries that make these concepts easily a accessible.

Space dyanmics software generally runs into the following pitfalls:
1. Expensive, commercially licensed and closed-source software 
2. High-fidelity heritage libraries making integration with modern tools challenging
3. Heritage APIs, while extremely well tested, also have an extremely steep learning curve

Students, researchers, and engineers frequently end up reimplementing common astrodynamics functions with an unforunate regularity. While reimplementation of common code can be a good learning mechansims, in most cases it is both error prone and something people could be spending other time on. This project seeks to address this challenge by providing an easy to use library, built in Rust for performance with bindings to Python for ease of use.

This project builds on experience from past projects in building space dynamics software:
- [brahe](https://github.com/duncaneddy/brahe) A pure-python astrodynamics library
- [SatelliteDynamics.jl](https://github.com/sisl/SatelliteDynamics.jl) A Julia astrodynamics library

### Usage

If you're using rastro for school, research, or commercial endeavour, I'd love 
to know about it! Tweet me [@duncaneddy](https://twitter.com/DuncanEddy)

## Installation

rastro can be used as either a standalone rust library or a python library.

### Python

```bash
pip install rastro
```

### Rust

```toml
[dependencies]
...
rastro = { version = "0.1.0" }
```

## License

The rastro package is licensed and distributed under the MIT License to encourage adoption and to make it easy to integrate with other tools.

The only thing asked is that if you do use the package in your work, or appreciate the project, either send a message or star the project. Knowing that the project is being actively used is a large motivator for continued development.