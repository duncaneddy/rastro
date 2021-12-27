# Installation

rastro can be used as either a standalone rust library or a python library. Rastro is implemented as a Rust library, but
we simultaneously provide a python wrapper to easily use the functionality.


### Rust

First ensure that you have Rust installed in your desired development environment. The [Rust Book](https://doc.rust-lang.org/book/ch01-01-installation.html)
is an excellent resource to start

To add RAstro to you desired librarysimly edit your `Cargo.toml` file and add `rastro` as a dependency.
```toml
[dependencies]
...
rastro = { version = "0.1.0" }
```

### Python

Currently, the python package is a source distribution meaning that it provides the rust source of the wrapper and compiles
the Python package at installation. While support for wheel distribution is desired it isn't planned for near-term implementation. 
A pull request for wheel support would be welcome.

The source distribution requires that a Rust toolchain is installed and available locally. Additionally, a Python environment
must be available. We recommend using [pyenv](https://github.com/pyenv/pyenv) and [pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv)
for managing Python environments.

```bash
pip install rastro
```
