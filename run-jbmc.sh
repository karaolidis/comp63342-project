#!/bin/bash

base_dir="./examples/benchmark"
num_threads=1

run_jbmc() {
    folder="$1"
    stdout_file="$folder/stdout.log"
    stderr_file="$folder/stderr.log"
    ./target/release/jbmc-counterexample -j ./cbmc/jbmc/src/jbmc/jbmc -t 30 "$folder" >"$stdout_file" 2>"$stderr_file"
}

export -f run_jbmc

cleanup() {
    echo "Cleaning up..."
    pkill -P $$
    exit 1
}

trap cleanup SIGINT SIGTERM

find "$base_dir" -maxdepth 1 -type d -print0 | xargs -0 -P "$num_threads" -I {} bash -c 'run_jbmc "{}"'
