#!/bin/bash

set -e -o pipefail

if [ ! -f ./deploy.bash ]; then
    echo 'This script must be run at misc/webgl-tutorial/ directory' 1>&2
    exit 1
fi

projects=(
    billboard
    cubemap
    filter
    framebuffer
    light
    point_and_line
    polygon
    quaternion
    shadowmap
    stencil
    texture_and_blending
    toon
    mirror
    hemisphere
)

echo "Building projects..."
npm install
npm run build

basedir="../docs/webgl"

echo "Preparing the base directory ${basedir} and assets"
rm -rf ${basedir}
mkdir -p ${basedir}
cp -R assets ${basedir}/
cp minMatrix.js ${basedir}/
cp cover.html ${basedir}/index.html

for project in "${projects[@]}"; do
    echo "Copying project: ${project}"
    mkdir -p "${basedir}/${project}"
    cp "${project}"/*.{html,js,frag,vert} "${basedir}/${project}/"
done

echo "Done. Open http://localhost:1234 in a web browser"
case "$OSTYPE" in
    darwin*)
        open "http://localhost:1234"
        ;;
esac
npx http-server ../docs/webgl/ -p 1234
