#!/bin/bash

set -e

if [ ! -d answer ]; then
    echo "Put './answer' directory by copying 'healed' directory from ziglings repository" 2>&1
    exit 1
fi

if [ "$1" = "--diff" ]; then
    shift
    show_diff=true
else
    show_diff=false
fi

set +e
diff="$(diff -u "$1" "./answer/$1" 2>&1)"
status=$?
if [ "$status" = 0 ]; then
    echo 'PASS!' >&2
else
    echo 'FAIL!' >&2
    if [ "$show_diff" = 'true' ]; then
        echo "$diff" >&2
    fi
fi
exit $status
