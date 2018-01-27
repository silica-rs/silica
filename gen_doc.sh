#!/bin/bash

RUST_TARGET_PATH=.. xargo doc --target cortex-m3 --no-deps
