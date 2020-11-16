#!/bin/sh

set -x

cd mylib-rs
cargo vendor --versioned-dirs cargo/vendor > .cargo/config
cargo raze

cd ../person-rs
cargo vendor --versioned-dirs cargo/vendor > .cargo/config
cargo raze
