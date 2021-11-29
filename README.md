# aoc-2021-rust
This repository will hold my solutions to the 2021 set of coding exercises for the "Advent of Code". I will be using the Rust programming language for this and trying to be as idiomatic as possible.

# Installing Rust #
In order to run my solutions, it's easiest to install Rust locally on your machine using Rustup. Rustup is the official toolchain installer for Rust and will install Cargo along with the Rust compiler. Depending on your operating system, you may need to install additional dependencies to build the Rust binaries. Notably, on Windows you need the C/C++ build tools.

In order to install Rustup, you can visit: https://rustup.rs/ and follow the instructions given for your platform. Alternatively other platforms such as Linux distributions may have their own way to install Cargo/Rust

Cargo will indicate any further dependencies required as you follow the next steps to run the binaries

# Running The Solutions #
Each day + part will have its own self-contained binary in a separate file. This means that all the code related to that individual exercise is in that file and you don't have to wade through lots of different files to find out how the code works.

I'll be copying the data that goes along with the exercise and embedding it into the file itself, so no need to get any external data files.

In order to run one of the binaries, you can use the following command at the command line (assuming you're inside the project directory)

```
cargo run day<day_number>-part<part_number>
```

These will be the names of the files also so will be easy to find. For example to run part 2 of day 3, we can do

```
cargo run day03-part2
```
