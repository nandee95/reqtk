#!/usr/bin/env bash
set -euo pipefail

if [[ -n "$(git status --porcelain)" ]]; then
    echo "WARNING: Git working tree has changes"
    exit 1
fi

cargo run -- convert requirements.req --to md -o REQUIREMENTS.md

if [[ -n "$(git status --porcelain)" ]]; then
    echo "WARNING: cargo fmt modified files."
    git --no-pager diff
    exit 1
fi