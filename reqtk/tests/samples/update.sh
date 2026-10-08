#!/usr/bin/env bash

DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

for file in "$DIR"/*.req; do
    [[ -f "$file" ]] || continue

    convert_json="${file%.req}.convert.json"
    cargo run -- convert "$file" --to json -o "$convert_json"

    tokenize_json="${file%.req}.tokenize.json"
    cargo run -- tokenize "$file" -f json -o "$tokenize_json"
    
    format_json="${file%.req}.format.req"
    cargo run -- format "$file" -o "$format_json"
done