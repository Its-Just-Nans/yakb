#!/bin/bash

# https://golb.n4n5.dev/utils-linux.html#oneliner-faviconico-generator

cd "$(dirname "$0")" || exit 1

TO_ICONIFY=yakb.svg
for i in 48 96 144 192; do convert -background none $TO_ICONIFY -resize ${i}x${i} favicon-${i}x${i}.png; done
convert -background none favicon-* favicon.ico

rm favicon-*.png
mv favicon.ico ../../../public/favicon.ico
cp $TO_ICONIFY ../../../public/favicon.svg

SIZE=1024
convert -background none $TO_ICONIFY -resize "${SIZE}x${SIZE}" yakb.png
