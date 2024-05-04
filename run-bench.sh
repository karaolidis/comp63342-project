#!/bin/bash

base_dir="./examples/benchmark"

num_folders=0

num_folders_with_timeout=0

num_folders_with_no_failures=0
num_folders_with_failures=0
num_folders_with_errors=0

total_assertion_failures=0

for folder in "$base_dir"/*/; do
    num_folders=$((num_folders + 1))
    stderr_file="${folder}stderr.log"
    stdout_file="${folder}stdout.log"

    if [[ -f "$stderr_file" ]]; then
        num_stderr=$((num_stderr + 1))

        if grep -q "timed out" "$stderr_file"; then
            num_folders_with_timeout=$((num_folders_with_timeout + 1))
            continue
        fi
    fi

    if [[ -f "$stdout_file" ]]; then
        num_failures=$(grep -o "Found [0-9]* assertion failure" "$stdout_file" | awk '{sum += $2} END {print sum}')
        if [[ -n "$num_failures" && "$num_failures" -gt 0 ]]; then
            total_assertion_failures=$((total_assertion_failures + num_failures))
            num_folders_with_failures=$((num_folders_with_failures + 1))
        fi

        if [[ "$num_failures" -eq 0 ]]; then
            num_folders_with_no_failures=$((num_folders_with_no_failures + 1))
        fi

        if grep -q "ERROR" "$stdout_file"; then
            num_folders_with_errors=$((num_folders_with_errors + 1))
        fi
    fi
done

echo "Total number of folders: $num_folders"

echo "Number of folders with timeout: $num_folders_with_timeout"

echo "Number of folders with no assertion failures: $num_folders_with_no_failures"
echo "Number of folders with at least one assertion failure: $num_folders_with_failures"
echo "Number of folders with at least one error: $num_folders_with_errors"

echo "Total number of assertion failures: $total_assertion_failures"
