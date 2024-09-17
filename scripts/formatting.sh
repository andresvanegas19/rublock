#!/bin/bash
rustfmt src/**/*.rs
cargo clippy
