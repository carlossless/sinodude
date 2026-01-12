#!/usr/bin/env bash
set -uo pipefail

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

# Run generate-part on each file individually, continuing on failure
FAILED=0
SUCCESS=0
FAILED_FILES=()
while IFS= read -r file; do
    if $GENERATE_PART --output-dir "$OUTPUT_DIR" "$file"; then
        ((SUCCESS++))
    else
        FAILED_FILES+=("$file")
        ((FAILED++)) || true
    fi
done <<< "$GPT_FILES"

echo
echo "Part definitions generated in $OUTPUT_DIR"
echo "Success: $SUCCESS, Failed: $FAILED"

if [[ ${#FAILED_FILES[@]} -gt 0 ]]; then
    echo
    echo "Failed files:"
    for f in "${FAILED_FILES[@]}"; do
        echo "  $f"
    done
fi
