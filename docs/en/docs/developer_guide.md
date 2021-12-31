# Developer Guide

This section provides information on how to get started developing new features and capabilities for
RAstro.

## Setup

To set up a local development environment follow the steps below

### MacOS

1. Ensure MacOS Command Line Tools are installed (if not already installed)

    ```bash
    xcode-select --install
    ```

2. Create a location to clone the RAstro repository to. I generally clone my repositories to a 
   `repos` folder in my home directory.

    ```bash
    mkdir ~/repos
    ```

3. Clone the RAstro repository locally

    ```bash
    cd ~/repos
    git clone git@github.com:duncaneddy/rastro.git
    cd rastro
    ```

4. Install [Rust](https://www.rust-lang.org)[^1]. The rust compiler and toolchain are needed to 
   compile the core rust library

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

5. Install the [Homebrew](https://brew.sh) package manager (if not already installed)

    ```bash
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    ```


5. Next we setup a Python 3.6+ virtual environment for isolated development of RAstro functionality

    1. Install [Pyenv](https://github.com/pyenv/pyenv) to manage multiple python environments

        ```bash
        brew install pyenv
        ```
       
        Assuming you are running MacOS's default zsh shell to complete the Pyenv setup you need 
       to run the following 
   
        ```bash
        echo 'eval "$(pyenv init --path)"' >> ~/.zprofile
        echo 'eval "$(pyenv init -)"' >> ~/.zshrc
        source ~/.zshrc
        ```

    2. Install [Pyenv-virtualenv](https://github.com/pyenv/pyenv-virtualenv) to create and manage virtual environments 

        ```bash
        brew install pyenv-virtualenv
        ```
       
        Similarly, after installation has completed execute the following command to activate 
       pyenv in your shell environment

        ```bash
        eval "$(pyenv virtualenv-init -)"
        ```

    3. Next we install the version of Python we want to use for our development environment

        ```bash
        pyenv install -l     # List versions available to isntall 
        pyenv install 3.8.12 # Select version for installation
        ```
    
    4. Once installation has completed create a 

        ```bash
        pyenv virtualenv 3.8.12 rastro  # You can rename it something other than `rastro` as well
        pyenv local rastro              # Set `rastro` as default python environment for shells 
                                        # started in the package directory 
        pip install -U pip              # Update pip because it's always out of date
        ```

6. Build the python package and install documentation dependencies with 

    ```bash
    ./docs/build.sh install
    ```

    If this one-step build command doesn't succeed, you may need to to an incremental 
   installation of the Python build dependencies

    ```bash
    pip install setuptools_rust
    cd rastro_python
    python setup.py install
    ```

7. The setup of the development environment is now complete. To confirm it you can build and 
   serve the package documetnation locally with 

    ```bash
    ./docs/build.sh
    ```
    
    This command will compile all code examples in the `docs/examples` directory for both Rust 
   and Python, as well as execute them to ensure that they properly execute with the current 
   development environment. It will also execute all scripts in the `docs/figures/` directory to 
   produce all figures.

    If changes have been made either to the underlying rust library or to the python package 
   that need to be reflected in the behavior of the python package you will need to run the 
   install command below to recompile the Python package:

    ```bash
    ./docs/build.sh install
    ```

### Linux

_**WIP**_

## Development Steps

When adding a new feature or capability to the RAstro package there are a few steps that should 
all be done to ensure that everything works properly and that the package quality remains high. 
This section will walk though the development of adding a fictious `transporter` module and 
function `beam_me_up_scotty` to RAstro. This example is a bit more intensive since it adds 
additional steps required to setup a new module.

1. Create the module in Rust
2. Add the Rust module to the library
3. Implement the desired functionality in Rust
4. Add tests in Rust
5. Add a wrapper module to rastro_python
6. Write a PyO3 wrapper to call Rust functionality from python
7. Add the function to the package `__init__.py` in rastro_python
8. Write tests of the Python functionality
9. Create a documentation page in the User guide section
10. Write example script in Rust and add it to the documentation
11. Write example script in Python and add it to the documentation
12. Write figure in Python and add it to the documentation
    1. Can be a Matplotlib figure
    2. Can be a dynamic Plotly figure
13. Update `CHANGELOG.md` with summary of change


[^1]: Steps taken from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)