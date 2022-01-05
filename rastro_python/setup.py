#!/usr/bin/env python

from setuptools import setup
from setuptools_rust import RustExtension

from os import path

if __name__ == "__main__":
    this_directory = path.abspath(path.dirname(__file__))
    with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
        long_description = f.read()

    setup(
        name="rastro",
        version="0.0.0", # Do NOT edit. Will be updated for release by CI pipeline
        classifiers=[
            "License :: OSI Approved :: MIT License",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
        ],
        packages=["rastro"],
        rust_extensions=[
            RustExtension("rastro.constants", debug=False),
            RustExtension("rastro.orbits", debug=False),
            RustExtension("rastro.eop", debug=False),
        ],
        include_package_data=True,
        zip_safe=False,
        long_description=long_description,
        long_description_content_type='text/markdown',
    )