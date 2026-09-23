#!/usr/bin/env bash

set -euo pipefail

RUNS=${1:-100}
WARMUP=${2:-10}
BIN=target/release/pdc-rust-check

echo "warming up..."

for ((i=1; i<=WARMUP; i++)); do
    "$BIN" >/dev/null
done

echo "benchmarking $RUNS runs..."

total_ns=0
min_ns=999999999999999999
max_ns=0

for ((i=1; i<=RUNS; i++)); do
    start=$(date +%s%N)

    "$BIN" >/dev/null

    end=$(date +%s%N)

    elapsed=$((end - start))

    total_ns=$((total_ns + elapsed))

    if (( elapsed < min_ns )); then
        min_ns=$elapsed
    fi

    if (( elapsed > max_ns )); then
        max_ns=$elapsed
    fi
done

avg_ns=$((total_ns / RUNS))

python3 - <<PY
runs = $RUNS
avg = $avg_ns
minimum = $min_ns
maximum = $max_ns

print(f"runs:    {runs}")
print(f"average: {avg / 1_000_000:.3f} ms")
print(f"min:     {minimum / 1_000_000:.3f} ms")
print(f"max:     {maximum / 1_000_000:.3f} ms")
PY