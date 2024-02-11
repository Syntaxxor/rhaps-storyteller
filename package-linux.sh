#!/bin/bash

mkdir ".packaging"
mkdir ".packaging/rhaps-storyteller"
cp "target/release/rhaps-storyteller" ".packaging/rhaps-storyteller"
cp -r "resources" ".packaging/rhaps-storyteller"
cp -r "stories" ".packaging/rhaps-storyteller"

tar -czf "packaged.tar.gz" -C ".packaging" .

rm -r ".packaging"