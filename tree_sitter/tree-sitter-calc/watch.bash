#!/bin/bash

set -e -o pipefail

event="$1"
path="$2"

if [[ "$event" != 'change' ]]; then
    exit
fi

set -x

case "$path" in
    grammar.js) npm run gen ;;
    example) npm run debug ;;
    *) ;;
esac
