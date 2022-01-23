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
</p>

----

Documentation: https://duncaneddy.github.io/rastro

Rust Library Reference: https://docs.rs/crate/rastro/latest 

Source Code: https://github.com/duncaneddy/rastro

----

# RAstro
rastro is a modern space dynamics library for research and engineering
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
astro = RAstro). Space dynamics is an expansive field that covers multiple
domains of mathematics, astrodynamics, attitude dynamics, and systems
engineering. While the underlying concepts have been studied and known since
Kepler wrote down his three laws, there are few modern software
libraries that make these concepts easily accessible. Space dynamics
software generally runs into the following pitfalls:

1. It is usually expensive, commercially licensed software making it
   inaccessible for students, hobby projects, or new endeavours.
2. They are heritage libraries making integration with modern tools challenging
3. They are heritage APIs, while extremely well tested, also have an extremely
   steep learning curve. Additionally, the closed-source nature makes it
   difficult, if not impossible, to understand the true algorithmic
   implementation. This can make root-causing issues and learning from the
   software prohibitive.

Due to these challenges, students, researchers, and engineers frequently end up
reimplementing common astrodynamics functions with an unfortunately frequent
regularity. While  reimplementation of common code can be a good learning
mechanisms, in most cases it is both error-prone and costs time better spent
on other endeavours. This project seeks to address this challenge by
providing an easy-to-use, well-tested library, built in Rust for
performance with bindings to Python for ease of use.

This project builds on experience from past projects in building space
dynamics software:
- [brahe](https://github.com/duncaneddy/brahe) A pure-python astrodynamics
  library
- [SatelliteDynamics.jl](https://github.com/sisl/SatelliteDynamics.jl) A
  Julia astrodynamics library

## Documentation

You can find the package documentation [here](https://duncaneddy.github.io/rastro).
This documentation is meant to provide a human-friendly walkthrough of the 
software and package. RAstro is currently in the early stages of development so
the documentation will likely not be complete. Sections marked **[WIP]** 
will have some software functionality implemented but not be considered 
documented. 

## Software Usage and License

The RAstro package is licensed and distributed under an [MIT License](https://github.com/duncaneddy/rastro/blob/main/LICENSE) to
encourage adoption and to make it easy to integrate with other tools.

The only thing asked is that if you do use the package in your work, or
appreciate the project, either send a message or star the project. Knowing
that the project is being actively used is a large motivator for continued
development.

### Support and Acknowledgement

RAstro is currently being developed primarily for my own enjoyment and 
because I find having these tools helpful in professional work. I plan to 
continue developing it for the time being regardless of greater adoption, 
time permitting.

That being said, it's incredibly encouraging and useful to know if the 
software is being adopted or found useful in wider practice. If you're 
using RAstro for school, research, or a commercial endeavour, I'd
love to know about it! Tweet me [@duncaneddy](https://twitter.com/DuncanEddy) or
email me at duncan.eddy (at) gmail.com.

I'd love to hear your feedback or thoughts!