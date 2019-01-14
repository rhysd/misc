#!/bin/bash

set -e

dir="$1"

if [[ "$1" == "" ]]; then
    echo 'Directory path must be given as first argument' 2>&1
    exit 1
fi
echo $dir


