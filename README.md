[![Check and Lint](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/check-and-lint.yaml)
[![Release Packaging](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/release-packaging.yaml/badge.svg)](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/release-packaging.yaml)
[![Test Coverage](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/test-coverage.yaml/badge.svg)](https://github.com/ForgottenMaster/aoc-2021/actions/workflows/test-coverage.yaml)
[![codecov](https://codecov.io/gh/ForgottenMaster/aoc-2021/branch/main/graph/badge.svg?token=9PWKPRF1UW)](https://codecov.io/gh/ForgottenMaster/aoc-2021)

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
Each solution when run will output, along with the part 1 and 2 answers, a timing of how long it took to complete.

For more detailed benchmarking, Criterion is added as a dev-dependency and benchmarking can be run as:

```
cargo bench
```

This will run all the benchmarks and can take a while to do so. To run a single solution you can pass the name to bench. For example, to run only day 1's solution:

```
cargo bench run_day_1
```

# Dependencies #
I won't be including any crates/libraries in the project for use in solving the solutions. I'll either be using the standard library functionality or rolling my own data structures where there are gaps.

I will however include Criterion as a dev-dependency. This means if you're running the binary/solutions it won't need to install it, but for benchmarking and testing it will install.

# Completed Solutions #
This table identifies which solutions have been completed and which are still yet to be done. Ideally these will get checked off at a rate of 2 per day to keep upto speed with release.

|Puzzle|Part 1|Part 2|Benchmark (microseconds)|
|:-:|:-:|:-:|;-;|
|[Day 1 - Sonar Sweep](https://adventofcode.com/2021/day/1)|:heavy_check_mark:|:heavy_check_mark:|489|
|[Day 2 - Dive!](https://adventofcode.com/2021/day/2)|:heavy_check_mark:|:heavy_check_mark:|201|
|[Day 3 - Binary Diagnostic](https://adventofcode.com/2021/day/3)|:heavy_check_mark:|:heavy_check_mark:|311|
|[Day 4 - Giant Squid](https://adventofcode.com/2021/day/4)|:heavy_check_mark:|:heavy_check_mark:|781|
|[Day 5 - Hydrothermal Venture](https://adventofcode.com/2021/day/5)|:heavy_check_mark:|:heavy_check_mark:|22,862|
|[Day 6 - Lanternfish](https://adventofcode.com/2021/day/6)|:heavy_check_mark:|:heavy_check_mark:|115|
|[Day 7 - The Treachery of Whales](https://adventofcode.com/2021/day/7)|:heavy_check_mark:|:heavy_check_mark:|571,260|
|[Day 8 - Seven Segment Search](https://adventofcode.com/2021/day/8)|:heavy_check_mark:|:heavy_check_mark:|398|
|[Day 9 - Smoke Basin](https://adventofcode.com/2021/day/9)|:heavy_check_mark:|:heavy_check_mark:|894|
|[Day 10 - Syntax Scoring](https://adventofcode.com/2021/day/10)|:heavy_check_mark:|:heavy_check_mark:|94|
|[Day 11 - Dumbo Octopus](https://adventofcode.com/2021/day/11)|:heavy_check_mark:|:heavy_check_mark:|167|
|[Day 12 - Passage Pathing](https://adventofcode.com/2021/day/12)|:heavy_check_mark:|:heavy_check_mark:|229,000|
|[Day 13 - Transparent Origami](https://adventofcode.com/2021/day/13)|:heavy_check_mark:|:heavy_check_mark:|3,190|
|[Day 14 - Extended Polymerization](https://adventofcode.com/2021/day/14)|:heavy_check_mark:|:heavy_check_mark:|877|
|[Day 15 - Chiton](https://adventofcode.com/2021/day/15)|:heavy_check_mark:|:heavy_check_mark:|1,145,300|
|[Day 16 - Packet Decoder](https://adventofcode.com/2021/day/16)|:heavy_check_mark:|:heavy_check_mark:|71|
|[Day 17 - Trick Shot](https://adventofcode.com/2021/day/17)|:heavy_check_mark:|:heavy_check_mark:|251|
|[Day 18 - Snailfish](https://adventofcode.com/2021/day/18)|:heavy_check_mark:|:heavy_check_mark:|226,500|
|[Day 19 - Beacon Scanner](https://adventofcode.com/2021/day/19)|:heavy_check_mark:|:heavy_check_mark:|225,400|
|[Day 20 - Trench Map](https://adventofcode.com/2021/day/20)|:heavy_check_mark:|:heavy_check_mark:|54,037|
|21|:x:|:x:|-|
|22|:x:|:x:|-|
|23|:x:|:x:|-|
|24|:x:|:x:|-|
|25|:x:|:x:|-|
