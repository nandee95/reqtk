#!/usr/bin/env bash
set -euo pipefail

if [[ -n "$(git status --porcelain)" ]]; then
    echo "WARNING: Git working tree has changes; using diffless format check."
    cargo fmt --all -- --check
    exit $?
fi

cargo fmt --all

if [[ -n "$(git status --porcelain)" ]]; then
    echo "WARNING: cargo fmt modified files."
    git --no-pager diff
    exit 1
fi