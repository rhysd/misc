#!/bin/bash

set -e

if [ ! -d ziglings ]; then
    echo 'Run git clone https://github.com/ratfactor/ziglings.git at first' 2>&1
    exit 1
fi
cp ./ziglings/exercises/"$1"_* .
