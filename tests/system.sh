#!/bin/sh
#system.sh ver. 20200812134300 Copyright 2020 alexx, MIT License
# rdfa.deps="[sh]"

usage(){
    printf "Usage: $(basename $0) [-h]
    -h  This help message
    \n";
    exit 0
}

[ "$1" ]&& echo "$1"|grep -q '\-h' && usage

cargo run -- $* -c 3x3.ini

