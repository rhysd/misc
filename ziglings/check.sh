#!/bin/bash

set -e

if [ ! -d answer ]; then
    echo "Put './answer' directory by copying 'healed' directory from ziglings repository" 2>&1
    exit 1
fi

set +e
diff="$(diff -u "$1" "./answer/$1" 2>&1)"
status=$?
if [ "$status" = 0 ]; then
    echo 'PASS!' >&2
else
    echo 'FAIL!' >&2
    if [ "$2" = "--diff" ]; then
        echo "$diff" >&2
    fi
fi
exit $status
