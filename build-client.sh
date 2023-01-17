#!/usr/bin/bash

cd client
npm install
npm run build
tar cf dist.tar.gz dist/
mv dist.tar.gz ../
