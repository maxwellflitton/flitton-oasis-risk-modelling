#!/usr/bin/env python
from setuptools import dist
dist.Distribution().fetch_build_eggs(['setuptools_rust'])
from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name="flitton-oasis-risk-modelling",
    version="0.1",
    rust_extensions=[RustExtension(
        ".flitton_oasis_risk_modelling.flitton_oasis_risk_modelling",
        path="Cargo.toml", binding=Binding.PyO3)],
    packages=["flitton_oasis_risk_modelling"],
    include_package_data=True,
    package_data={'': ['*.csv']},
    classifiers=[
            "License :: OSI Approved :: MIT License",
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "Programming Language :: Python",
            "Programming Language :: Rust",
            "Operating System :: POSIX",
            "Operating System :: MacOS :: MacOS X",
        ],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    entry_points={
        'console_scripts': [
            'fib-number = flitton_fib_rs.'
            'fib_number_command:'
            'fib_number_command',
            'config-fib = flitton_fib_rs.'
            'config_number_command:'
            'config_number_command',
        ],
    },
    requirements=[
        "pyyaml>=3.13",
        "numpy"
    ]
)
