#!/bin/bash
# Usage: ./scripts/embed-exercises.sh exercises/00-declarations
# Outputs markdown with collapsible <details> blocks for each exercise .rs file.

set -euo pipefail

section_dir="$1"
bin_dir="$section_dir/src/bin"

if [ ! -d "$bin_dir" ]; then
    echo "Error: $bin_dir not found" >&2
    exit 1
fi

for f in $(ls "$bin_dir"/*.rs | sort); do
    name=$(basename "$f" .rs)
    # Turn 01_literal into "01 · literal", replacing underscores after the number
    num="${name%%_*}"
    rest="${name#*_}"
    label="$num · ${rest//_/ }"

    echo '<details markdown="1">'
    echo "<summary>$label</summary>"
    echo ""
    echo '```rust'
    cat "$f"
    echo '```'
    echo ""
    echo "</details>"
    echo ""
done
