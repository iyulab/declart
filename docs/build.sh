#!/usr/bin/env bash
# Copies spec/** into docs/src/ then builds the mdBook.
# Run from the repo root: bash docs/build.sh
set -euo pipefail

cp spec/principles.md docs/src/principles.md
cp spec/schema.md docs/src/schema.md
mkdir -p docs/src/kinds
for f in spec/kinds/*.md; do
    cp "$f" "docs/src/kinds/$(basename "$f")"
done

mdbook build docs/
