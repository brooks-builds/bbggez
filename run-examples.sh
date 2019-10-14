#!/usr/bin/env sh

for example in $(ls examples)
do
  echo "Running example $example"
  cargo run --example "$(echo "$example" | cut -f 1 -d ".")"
done
