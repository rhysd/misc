#!/bin/bash

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
