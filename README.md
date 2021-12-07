# aoc-2021-rust
This repository will hold my solutions to the 2021 set of coding exercises for the "Advent of Code". I will be using the Rust programming language for this and trying to be as idiomatic as possible.

# Installing Rust #
In order to run my solutions, it's easiest to install Rust locally on your machine using Rustup. Rustup is the official toolchain installer for Rust and will install Cargo along with the Rust compiler. Depending on your operating system, you may need to install additional dependencies to build the Rust binaries. Notably, on Windows you need the C/C++ build tools.

In order to install Rustup, you can visit: https://rustup.rs/ and follow the instructions given for your platform. Alternatively other platforms such as Linux distributions may have their own way to install Cargo/Rust

Cargo will indicate any further dependencies required as you follow the next steps to run the binaries

# Running The Solutions #
Each day will have its own module named after it such as day01, day02, etc.

There will be a single binary that can be run with the number of the day whose solution you'd like to run. This is so that:

1. The single binary can wrap the functionality to add the printing and timing logic of the run
2. It's easier to type the command to invoke it
3. We can then test the logic from a benchmarking suite if we want to in future

To run a specific day, just type the number after the "cargo run" command. This is **not** padded with 0's so the following are valid examples:

```
cargo run --release 1
cargo run --release 9
cargo run --release 12
```

# Benchmarks #
Each solution when run will output, along with the part 1 and 2 answers, a timing of how long it took to complete. This is a crude
benchmark and isn't as reliable as using a benchmarking crate such as criterion, but gives a general idea of the time it takes.

Please bear in mind that the timings also include file I/O and not just the algorithm itself.

# Dependencies #
I won't be including any dependencies in the project unless absolutely necessary, for example if a problem
outputs an image with the answer in it. Any functionality I add on top of the standard library will be hand-written
by myself.

# Completed Solutions #
This table identifies which solutions have been completed and which are still yet to be done. Ideally these will get checked off at a rate of 2 per day to keep upto speed with release.

|Day|Part 1|Part 2|
|:-:|:-:|:-:|
|01|:heavy_check_mark:|:heavy_check_mark:|
|02|:heavy_check_mark:|:heavy_check_mark:|
|03|:heavy_check_mark:|:heavy_check_mark:|
|04|:heavy_check_mark:|:heavy_check_mark:|
|05|:heavy_check_mark:|:heavy_check_mark:|
|06|:x:|:x:|
|07|:x:|:x:|
|08|:x:|:x:|
|09|:x:|:x:|
|10|:x:|:x:|
|11|:x:|:x:|
|12|:x:|:x:|
|13|:x:|:x:|
|14|:x:|:x:|
|15|:x:|:x:|
|16|:x:|:x:|
|17|:x:|:x:|
|18|:x:|:x:|
|19|:x:|:x:|
|20|:x:|:x:|
|21|:x:|:x:|
|22|:x:|:x:|
|23|:x:|:x:|
|24|:x:|:x:|
|25|:x:|:x:|
