#!/bin/bash

set -e -o pipefail

dir="$(basename "$PWD")"
if [[ "$dir" != "mhwsgac" ]]; then
    echo "This script must be run at 'mhwsgac' directory" 1>&2
    exit 1
fi

hash="$(git rev-parse HEAD)"

set -x

npm run build

cp ./index.js ../docs/mhwsgac/index.js
cp ./index.html ../docs/mhwsgac/index.html
cp ./style.css ../docs/mhwsgac/style.css
cp ./node_modules/@picocss/pico/css/pico.min.css ../docs/mhwsgac/pico.css

git add ../docs/mhwsgac
git commit -m "mhwsgac: deploy from ${hash}"
