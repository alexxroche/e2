#!/bin/sh
# do.sh ver. 20201004171218 Copyright 2020 alexx, MIT License
# rdfa:deps="[cargo rustc]"

usage(){
    printf "Usage: $(basename $0) [-h]
    -h  This help message
    \n";
    exit 0
}

[ "$1" ]&& echo "$1"|grep -q '\-h' && usage


# for f in fmt check test build clippy;do cargo $f --release ; done && \
# cargo clippy --release
cargo run --release || cargo build --release && cargo run --release

