#!/usr/bin/env bash


cargo build --all-targets #requests for openssl static libraries
#RUST_BACKTRSCE=full cargo build --target x86_64-unknown-linux-gnu  --release 