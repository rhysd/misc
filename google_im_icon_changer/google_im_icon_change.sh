#! /bin/sh
set -e

pushd /Library/Input\ Methods/GoogleJapaneseInput.app/Contents/Resources/
for bak in `ls *.png *.tiff`; do
    cp $bak $bak.org
done
popd
cp *.png /Library/Input\ Methods/GoogleJapaneseInput.app/Contents/Resources/
pushd /Library/Input\ Methods/GoogleJapaneseInput.app/Contents/Resources/
for png in `ls *.png`; do
    mv $png ${png%.*}.tiff
done
popd
cp *.png /Library/Input\ Methods/GoogleJapaneseInput.app/Contents/Resources/
echo 'done.'
