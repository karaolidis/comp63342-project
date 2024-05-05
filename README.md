# COMP63342-project Overview and Instructions

Rust is superior.

## Overview

This project aims to automate benchmarks for Java Bytecode Model Checking (JBMC). The repository includes Rust and Python files that facilitate parsing, code generation, and executing model checks, as well as Bash scripts to handle batch processing and benchmarking.

## Directory Structure

- `runner.rs`: Module for controlling the overall model checking process.
- `codegen.rs`: Module responsible for generating JBMC code.
- `main.rs`: Entry point of the Rust-based checker.
- `translate.py`: Python utility for additional translation needs.

## Prerequisites

1. **Rust**: Ensure you have a recent Rust toolchain.
2. **Python**: A Python 3.x environment with necessary dependencies.
3. **JBMC**: Ensure JBMC is properly installed and accessible via system path.

## How to Use

cd to the project's folder then:

1. Build the Rust Code:

   ```bash
   cargo +nightly build --release
   ```

2. Run JBMC:

   Edit `run-jbmc.sh` to adjust the `base_dir` and `num_threads` if needed. Execute:

   ```bash
   ./target/release/jbmc-counterexample -h
   ```

3. Aggregating:

   Execute `run-bench.sh`:

   ```bash
   ./run-bench.sh
   ```

## Further Information

- **Logging**: Check `stdout.log` and `stderr.log` in each benchmark folder for detailed output.
