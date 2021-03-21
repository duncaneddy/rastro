#!/usr/bin/env python
import os

from setuptools import setup
from setuptools_rust import RustExtension

from os import path
this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(
    name="rastro",
    version="0.1.4",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    packages=["rastro"],
    rust_extensions=[
        RustExtension("rastro_python.constants"),
        RustExtension("rastro_python.orbits")
    ],
    include_package_data=True,
    zip_safe=False,
    long_description=long_description,
    long_description_content_type='text/markdown',
)