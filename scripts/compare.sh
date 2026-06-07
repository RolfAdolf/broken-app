#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts/benches

# до оптимизаций
cargo bench --bench criterion -- --save-baseline before | tee artifacts/benches/baseline_before.txt

# после оптимизаций:
# cargo bench --bench criterion -- --baseline before | tee artifacts/benches/baseline_after.txt
