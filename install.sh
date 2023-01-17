#!/usr/bin/bash

rm -rf dist
mkdir dist

# Build Client
cd client
npm install
npm run build
mv dist ../dist/static
cd ../

# Build Server
cargo build --release
mv target/release/flux-playground dist/

# Move examples
cp -r examples dist/examples
