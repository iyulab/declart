#!/usr/bin/env bash
# Copies spec/** into docs/src/, builds declart-wasm for the playground, then builds the mdBook.
# Run from the repo root: bash docs/build.sh
set -euo pipefail

cp spec/principles.md docs/src/principles.md
cp spec/schema.md docs/src/schema.md
mkdir -p docs/src/kinds
for f in spec/kinds/*.md; do
    cp "$f" "docs/src/kinds/$(basename "$f")"
done

echo "Building WASM for playground..."
wasm-pack build crates/declart-wasm --target web --out-dir "$(pwd)/docs/src/playground/wasm" --release

mdbook build docs/
