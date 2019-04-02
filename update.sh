#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust --target riscv -i k210.svd
form -i lib.rs -o src
rm lib.rs
cargo fmt
