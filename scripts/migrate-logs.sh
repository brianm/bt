#!/bin/bash
#
# Migrate task files from old log entry format to new format.
#
# Old format: ### 2025-11-27T03:33:45Z Author Name
# New format: ---
#             # Log: 2025-11-27T03:33:45Z Author Name
#
# Usage: ./scripts/migrate-logs.sh [.tasks directory]
#
# The script matches the specific pattern "### {ISO-8601-timestamp} {author}"
# to avoid accidentally converting markdown headings in content.

set -e

TASKS_DIR="${1:-.tasks}"

if [ ! -d "$TASKS_DIR" ]; then
    echo "Error: Tasks directory '$TASKS_DIR' not found" >&2
    exit 1
fi

# ISO-8601 timestamp pattern: YYYY-MM-DDTHH:MM:SSZ
TIMESTAMP_PATTERN='[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z'

# Find all markdown files in the tasks directory
find "$TASKS_DIR" -name "*.md" -type f | while read -r file; do
    # Check if file contains old format log entries
    if grep -qE "^### ${TIMESTAMP_PATTERN} " "$file" 2>/dev/null; then
        echo "Migrating: $file"

        # Use sed to replace the pattern
        # macOS sed requires different syntax, so we use a temp file approach
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS BSD sed
            sed -E "s/^### (${TIMESTAMP_PATTERN} .*)$/---\\"$'\n'"# Log: \\1/" "$file" > "$file.tmp"
        else
            # GNU sed
            sed -E "s/^### (${TIMESTAMP_PATTERN} .*)$/---\n# Log: \\1/" "$file" > "$file.tmp"
        fi

        mv "$file.tmp" "$file"
    fi
done

echo "Migration complete."
