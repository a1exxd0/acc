<div>
    <p align="left">
        <img src="https://github.com/rust-lang/rust-artwork/blob/master/2019-RustConf/lucy-driving-dune-buggy.png" width="800">
    </p>
</div>

[![Test](https://github.com/a1exxd0/acc/actions/workflows/rust.yml/badge.svg)](https://github.com/a1exxd0/acc/actions/workflows/rust.yml)
[![Docs](https://github.com/a1exxd0/acc/actions/workflows/pages.yml/badge.svg)](https://github.com/a1exxd0/acc/actions/workflows/pages.yml)
![](https://img.shields.io/github/license/a1exxd0/acc)
![](https://img.shields.io/badge/made_for-UNIX-lightgrey)
![](https://img.shields.io/badge/Architecture-x86--64-blue)

`acc` is a C90 compiler built in Rust.


### Table of contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Testing](#testing)
- [Project goals](#project-goals)

## Installation
The project is very straightforward to build and as of right now requires no extra installs outside of the `rust` toolchain.
```rust
cargo build --release
cargo install --path .
```

## Usage
In an attempt to make this as easy to use as possible, the project will implement a small subset of the `gcc` API:
<div>
    <p align="left">
        <img src="https://github.com/a1exxd0/acc/blob/main/.github/assets/help-screen.png?raw=true" width="800">
    </p>
</div>
For example, a well-formed call would be:
```sh
acc main.c -o main -O3
```

## Features

## Testing

## Project goals