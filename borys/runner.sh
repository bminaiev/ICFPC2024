#!/usr/bin/bash

cd borys
RUST_BACKTRACE=1 cargo run "${@}"