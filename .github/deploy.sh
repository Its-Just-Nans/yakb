#!/bin/bash

npm install --force
npm run build


mkdir -p dist/animations/

rustup update stable
rustup default stable
rustup set profile minimal
rustup target add wasm32-unknown-unknown


wget -qO- https://github.com/thedodd/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

cd yakb-rs
if [ -f "../trunk" ]; then
    ../trunk build --release --public-url "./"
else
    trunk build --release --public-url "./"
fi
cd ..
mv yakb-rs/dist/* dist/animations/

