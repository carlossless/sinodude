#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
GENERATE_PART="$SCRIPT_DIR/generate-part/target/release/generate-part"
OUTPUT_DIR="$PROJECT_ROOT/src/parts"

# Check if generate-part binary exists
if [[ ! -x "$GENERATE_PART" ]]; then
    echo "Building generate-part..."
    (cd "$SCRIPT_DIR/generate-part" && cargo build --release)
fi

# Find all .gpt files in the project (case-insensitive)
GPT_FILES=$(find "$PROJECT_ROOT" -iname "*.gpt" -type f)

if [[ -z "$GPT_FILES" ]]; then
    echo "No .gpt files found"
    exit 0
fi

echo "Found GPT files:"
echo "$GPT_FILES"
echo

# Run generate-part on all files
$GENERATE_PART --output-dir "$OUTPUT_DIR" $GPT_FILES

echo
echo "Part definitions generated in $OUTPUT_DIR"
