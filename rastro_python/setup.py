#!/usr/bin/env python

from setuptools import setup,find_packages
from setuptools_rust import RustExtension, Binding

from os import path

if __name__ == "__main__":
    this_directory = path.abspath(path.dirname(__file__))
    with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
        long_description = f.read()

    setup(
        name="rastro",
        version="0.0.0", # Do NOT edit. Will be updated for release by CI pipeline
        # packages=["rastro"],
        packages=find_packages(),
        install_requires=[
            'typer>=0.4.0,<1.0.0',
            'rich>=11.0.0,<12.0.0',
            'numpy>=1.3.0,<2.0.0'
        ],
        classifiers=[
            "License :: OSI Approved :: MIT License",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
        ],
        rust_extensions=[
            RustExtension("rastro.module", binding=Binding.PyO3, debug=False),
        ],
        entry_points={
            'console_scripts': [
                'rastro = rastro.cli.__main__:main'
            ]
        },
        include_package_data=True,
        zip_safe=False,
        long_description=long_description,
        long_description_content_type='text/markdown',
    )