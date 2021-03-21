#!/usr/bin/env python
import sys

from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="rastro",
    version="0.1.2",
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
)