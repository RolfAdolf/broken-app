#!/usr/bin/env bash
set -euo pipefail

cargo build --release --bin demo
sudo dtrace -x ustackframes=100 -n 'profile-997 /pid == $target/ { @[ustack()] = count(); }' -c ./target/release/demo
