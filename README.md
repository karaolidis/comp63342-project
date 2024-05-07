# COMP63342-project Overview and Instructions

```
         #         #​
          #       #​
           #######​
         ##       ##           .-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-.​
         ##       ##           |  _________                      __                _________ __         .__ __             |​
           #######             !  \_   ___ \  ____  __ __  _____/  |_  ___________/   _____//  |________|__|  | __ ____    !​
         ###########           :  /    \  \/ /  _ \|  |  \/    \   __\/ __ \_  __ \_____  \\   __\_  __ \  |  |/ // __ \   :​
      ###   #####   ###        :  \     \___(  <_> )  |  /   |  \  | \  ___/|  | \/        \|  |  |  | \/  |    <\  ___/   :​
    ## #      #      # ##      !   \______  /\____/|____/|___|  /__|  \___  >__| /_______  /|__|  |__|  |__|__|_ \\___  >  !​
  #    # ########### #    #    |          \/                  \/          \/             \/                     \/    \/   |​
  ######             ######    .-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-.​
        ###       ###​
           #######​
```

## Overview

This project aims to automate benchmarks for Java Bytecode Model Checking (JBMC). The repository includes Rust and Python files that facilitate parsing, code generation, and executing model checks, as well as Bash scripts to handle batch processing and benchmarking.

## Files

- **`main.rs`**:

  The main entry point of the Rust application. It handles CLI arguments and manages overall functionality.

- **`runner.rs`**:

  Provides the execution framework to coordinate analysis tasks.

- **`codegen.rs`**:

  Handles the code generation logic for interpreting input commands and generating executable instructions.

- **`translate.py`**:

  Python script for actual translation using an external API or machine learning model.

  Used to translate code formats from the International Competition on Software Verification Benchmark tests , into a format which our tool can process.

## Prerequisites

1. **Rust**: Ensure you have a recent Rust toolchain.
2. **Python**: A Python 3.x environment with necessary dependencies.
3. **JBMC**: Ensure JBMC is properly installed and accessible via system path.

## Installation

1. Make sure you have Rust and Python installed on your system.
2. Clone the repository to your local machine.
3. Navigate to the project directory.
4. Install Rust dependencies using the nightly toolchain:

   - Run `rustup install nightly`

5. Compile and build the project in release mode using `cargo +nightly build --release`

## Usage

- Run the tool using:

  `cargo +nightly run --release -- [OPTIONS] [FILE_PATHS]...`

- Arguments:

  - `[FILE_PATHS]`: Path(s) to Java files/folders for analysis. The main file should be `Main.java` or match the folder name.

- Options:

  - `-e, --entrypoint <ENTRYPOINT>`: The entrypoint function to analyze (default: `test`).
  - `-c, --javac <JAVAC_PATH>`: Path to the Java compiler (default: `javac`).
  - `-j, --jbmc <JBMC_PATH>`: Path to the JBMC executable (default: `jbmc`).
  - `-t, --timeout <TIMEOUT>`: Execution timeout in seconds (default: `60`).
  - `-n, --threads <THREADS>`: Maximum number of threads to use (default: `1`).
  - `-h, --help`: Print help information.
  - `-V, --version`: Print version information.

## Examples

- Analyze a single file inside a folder:

  `cargo +nightly run --release -- path/to/java_folder`

- Analyze all subfolders (and their included files) inside a folder:

  `cargo +nightly run --release -- path/to/java_folder/*`

- Specifying `jbmc`` location, thread number and timeout limit:

  `cargo +nightly run --release -- -j ./cbmc/jbmc/src/jbmc/jbmc -n 4 -t 15 ./examples/benchmark/*`

- For Windows users, since the use of wildcard file expansion does not work, the alternative for multi-file execution is the following:

  - make soure you modify the `base-dir` of you folder containing example subfolders, and edit `num_threads` if needed.
  - esnure JBMC is in the Environment path.
  - navigate to the main project folder, then using Bash execute the script:

    ```
    bash winrun-jbmc.sh
    ```
