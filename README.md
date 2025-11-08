<p align="center">
  <a href="https://github.com/duncaneddy/rastro/"><img src="https://raw.githubusercontent.com/duncaneddy/rastro/main/docs/en/docs/img/logo-gold.png" alt="RAstro"></a>
</p>
<p align="center">
    <em>FastAPI framework, high performance, easy to learn, fast to code, ready for production</em>
</p>
<p align="center">
<a href="https://github.com/duncaneddy/rastro/actions/workflows/test.yml" target="_blank">
    <img src="https://github.com/duncaneddy/rastro/actions/workflows/test.yml/badge.svg" alt="Test">
</a>
<a href="https://crates.io/crates/rastro" target="_blank">
    <img src="https://img.shields.io/crates/v/rastro.svg" alt="Crate">
</a>
<a href="https://pypi.org/project/rastro" target="_blank">
    <img src="https://img.shields.io/pypi/v/rastro?color=blue" alt="PyPi">
</a>
<a href="https://duncaneddy.github.io/rastro" target="_blank">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg" alt="Docs">
</a>
<a href="https://github.com/duncaneddy/rastro/blob/main/LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/License-MIT-green.svg", alt="License">
</a>
</p>

----

Documentation: https://duncaneddy.github.io/rastro

Rust Library Reference: https://docs.rs/crate/rastro/latest

Source Code: https://github.com/duncaneddy/rastro

----

> [!WARNING] **Deprecation Notice**
>
> As of November 7, 2025 RAstro is being deprecated in favor of [brahe](https://github.com/duncaneddy/brahe).
>
> A complete rewrite of brahe was completed as of the v0.1.2 release which uses the Rust-Python approach of RAstro 
> with more extensive functionality, testing, and documentation.
>
> The project will be archived to remain available for reference, but no further development will occur.

# RAstro
RAstro is a modern satellite dynamics library for research and engineering
applications. It is designed to be high performance, easy to learn, and
fast to code, and ready for flight.

The key features are:

- **Intuitive**: API designed to be easily composable, making it easy to 
  solve complex problems correctly by building on core functionality.
- **Easy to Learn**: Designed to be easy to use and learn. Less time reading 
  papers, more time building.
- **Fast to code**: Rastro provides a Python 3.6+ wrapper that is
  auto-generated from the core rust libraries. Making it easy to use without
  compromising performance.
- **Fast to run**: Very high performance, on par with C++ libraries, thanks
  to core library being written in Rust.

RAstro gets its name from the combination of Rust and astrodynamics (Rust + 
astrodynamics = RAstro). The library specifically focuses on satellite astrodynamics
and space mission analysis. While the underlying concepts have been studied and known since 
Kepler wrote down his three laws, there are few modern software 
libraries that make these concepts easily accessible. While extremely well tested,
other astrodynamics and mission analysis software can have an extremely steep 
learning curve, making it difficult to quickly run simple analysis that is known
to be correct.

Because of this, students, researchers, and engineers frequently end up 
reimplementing common astrodynamics and mission analysis tools with unfortunately 
frequent regularity. While  reimplementation of common code can be a good learning 
mechanisms, in most cases it is both error-prone and costs time better spent 
on other endeavours. This project seeks to providing an easy-to-use, 
well-tested library, to enable everyone to more easily, and quickly 
perform astrodynamics and space mission analysis without sacrificing performance
or correctness. The software built in Rust for performance with bindings to 
Python for ease of use.

The implementation approach is opinionated, RAstro is not intended to implement
_every_ astrodynamics model and function that exists, but instead provide accurate,
contemporary and commonly used functions that will address most use-cases. One example of this is that the built-in Earth reference
frame transformation utilize the IAU 2006/2000A precession nutation model. However,
if a desired model isn't implemented because RAstro is open source users are free
to extend the software to address and functionality or modeling gaps that
exist to address their specific application.

This project builds on experience from past projects in building space 
dynamics software:
- [brahe](https://github.com/duncaneddy/brahe) A pure-python astrodynamics 
  library
- [SatelliteDynamics.jl](https://github.com/sisl/SatelliteDynamics.jl) A 
  Julia astrodynamics library

[//]: # (## Requirements)
[//]: # ()
[//]: # (## Installation)
[//]: # ()
[//]: # (## Example)
[//]: # ()
[//]: # (### Setup)
[//]: # ()
[//]: # (### Execution)
[//]: # ()
[//]: # (### Vizualization)
## Documentation

You can find the package documentation [here](https://duncaneddy.github.io/rastro).
This documentation is meant to provide a human-friendly walk through of the
software and package. RAstro is currently in the early stages of development so
the documentation will likely not be complete. Sections marked **[WIP]**
will have some software functionality implemented but not be considered
documented.

The most complete API reference guide will always be the Rust crate API 
reference, found on [crates.io](https://docs.rs/rastro/). This is always up-to-date with the latest release 
since it is autogenerated at build time during the release process.

## Software Usage and License

The RAstro package is licensed and distributed under an [MIT License](https://github.com/duncaneddy/rastro/blob/main/LICENSE) to
encourage adoption and to make it easy to integrate with other tools.

The only thing asked is that if you do use the package in your work, or
appreciate the project, either send a message or star the project. Knowing
that the project is being actively used is a large motivator for continued
development.

### Support and Acknowledgement

RAstro is currently being developed primarily for my own enjoyment and
because I find having these tools helpful in professional and hobby work. I plan to
continue developing it for the time being regardless of greater adoption as time permitting.

That being said, it's incredibly encouraging and useful to know if the
software is being adopted or found useful in wider practice. If you're
using RAstro for school, research, or a commercial endeavour, I'd
love to know about it! Tweet me [@duncaneddy](https://twitter.com/DuncanEddy) or
email me at duncan.eddy (at) gmail.com.

I'd love to hear your feedback or thoughts!