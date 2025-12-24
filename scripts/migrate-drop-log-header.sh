#!/bin/bash
#
# Migrate task files to format version 1:
# - Remove the ## Log section header
# - Add yatl_version: 1 to frontmatter
#
# Old format (version 0):
#   ---
#   title: Task title
#   id: abc12345
#   ...
#   ---
#   body content
#
#   ---
#   ## Log
#
#   ---
#   # Log: 2025-11-27T03:33:45Z author
#
# New format (version 1):
#   ---
#   yatl_version: 1
#   title: Task title
#   id: abc12345
#   ...
#   ---
#   body content
#
#   ---
#   # Log: 2025-11-27T03:33:45Z author
#
# Usage: ./scripts/migrate-drop-log-header.sh [.tasks directory]
#

set -e

TASKS_DIR="${1:-.tasks}"

if [ ! -d "$TASKS_DIR" ]; then
    echo "Error: Tasks directory '$TASKS_DIR' not found" >&2
    exit 1
fi

count=0

# Find all markdown files in the tasks directory
find "$TASKS_DIR" -name "*.md" -type f | while read -r file; do
    needs_migration=false

    # Check if file needs log header migration
    if grep -q "^## Log$" "$file" 2>/dev/null; then
        needs_migration=true
    fi

    # Check if file needs version field added
    if ! grep -q "^yatl_version:" "$file" 2>/dev/null; then
        needs_migration=true
    fi

    if [ "$needs_migration" = true ]; then
        echo "Migrating: $file"

        # Step 1: Remove the "## Log" section header
        # Pattern: ---\n## Log\n+--- becomes just ---
        if grep -q "^## Log$" "$file" 2>/dev/null; then
            if [[ "$OSTYPE" == "darwin"* ]]; then
                perl -i -0pe 's/---\n## Log\n+---/---/g' "$file"
            else
                sed -i -z 's/---\n## Log\n\+---/---/g' "$file"
            fi
        fi

        # Step 2: Add yatl_version: 1 after the opening ---
        if ! grep -q "^yatl_version:" "$file" 2>/dev/null; then
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # macOS: insert after first line (the opening ---)
                sed -i '' '1a\
yatl_version: 1
' "$file"
            else
                # GNU sed
                sed -i '1a yatl_version: 1' "$file"
            fi
        fi

        ((count++)) || true
    fi
done

echo "Migration complete."
