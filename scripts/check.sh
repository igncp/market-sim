#!/usr/bin/env bash

set -e

cargo test --all
cargo check --all
cargo clippy --all -- -D warnings
