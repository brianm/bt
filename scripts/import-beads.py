#!/usr/bin/env python3
"""Import Beads JSONL data into bt format."""
import json
import random
import re
import sys
from pathlib import Path
from datetime import datetime

# Crockford base32 alphabet (matches bt's id.rs)
CROCKFORD = "0123456789abcdefghjkmnpqrstvwxyz"


def generate_id():
    """Generate an 8-char Crockford base32 ID (40 bits)."""
    bits = random.getrandbits(40)
    id_chars = []
    for _ in range(8):
        id_chars.append(CROCKFORD[bits & 0x1f])
        bits >>= 5
    return "".join(reversed(id_chars))


def convert_priority(p):
    return {0: "critical", 1: "critical", 2: "high", 3: "medium"}.get(p, "medium")


def format_timestamp(ts):
    """Convert ISO timestamp to bt format (UTC)."""
    if not ts:
        return datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
    # Parse and reformat to ensure consistent format
    # Handle various fractional second lengths (Python < 3.11 is strict)
    ts = ts.replace("Z", "+00:00")
    # Normalize fractional seconds to 6 digits
    ts = re.sub(r"\.(\d+)([+-])", lambda m: f".{m.group(1)[:6].ljust(6, '0')}{m.group(2)}", ts)
    dt = datetime.fromisoformat(ts)
    return dt.strftime("%Y-%m-%dT%H:%M:%SZ")


def build_body(bead):
    """Combine description, design, and notes into body."""
    parts = []
    if bead.get("description"):
        parts.append(bead["description"])
    if bead.get("design"):
        parts.append(f"## Design\n\n{bead['design']}")
    if bead.get("notes"):
        parts.append(f"## Notes\n\n{bead['notes']}")
    return "\n\n".join(parts)


def to_markdown(task):
    """Convert task dict to bt markdown format."""
    fm = task["frontmatter"]
    lines = ["---"]
    escaped_title = fm["title"].replace('"', '\\"')
    lines.append(f'title: "{escaped_title}"')
    lines.append(f"id: {fm['id']}")
    lines.append(f"created: {fm['created']}")
    lines.append(f"updated: {fm['updated']}")
    lines.append(f"priority: {fm['priority']}")
    if fm.get("tags"):
        lines.append(f"tags: [{', '.join(fm['tags'])}]")
    if fm.get("blocked_by"):
        lines.append(f"blocked_by: [{', '.join(fm['blocked_by'])}]")
    lines.append("---")
    if task.get("body"):
        lines.append("")
        lines.append(task["body"])
    return "\n".join(lines)


def main():
    if len(sys.argv) < 2:
        print("Usage: import-beads.py <beads.jsonl> [output_dir]")
        sys.exit(1)

    jsonl_path = Path(sys.argv[1])
    output_dir = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(".")
    tasks_dir = output_dir / ".tasks"

    # Create directory structure
    (tasks_dir / "open").mkdir(parents=True, exist_ok=True)
    (tasks_dir / "closed").mkdir(parents=True, exist_ok=True)
    (tasks_dir / "in-progress").mkdir(parents=True, exist_ok=True)
    (tasks_dir / "blocked").mkdir(parents=True, exist_ok=True)
    (tasks_dir / "cancelled").mkdir(parents=True, exist_ok=True)

    # First pass: read all beads and create ID mapping
    beads = []
    with open(jsonl_path) as f:
        for line in f:
            if line.strip():
                beads.append(json.loads(line))

    id_map = {}  # beads_id -> bt_id
    for bead in beads:
        id_map[bead["id"]] = generate_id()

    # Second pass: convert and write tasks
    for bead in beads:
        bt_id = id_map[bead["id"]]

        # Map blocked_by dependencies
        blocked_by = []
        for dep in bead.get("dependencies", []):
            if dep.get("type") == "blocks" and dep.get("depends_on_id") in id_map:
                blocked_by.append(id_map[dep["depends_on_id"]])

        task = {
            "frontmatter": {
                "title": bead["title"],
                "id": bt_id,
                "created": format_timestamp(bead.get("created_at")),
                "updated": format_timestamp(bead.get("updated_at")),
                "priority": convert_priority(bead.get("priority", 2)),
                "tags": [bead["issue_type"]] if bead.get("issue_type") else [],
                "blocked_by": blocked_by,
            },
            "body": build_body(bead),
        }

        # Determine directory based on status
        status = bead.get("status", "open")
        status_dir = "closed" if status == "closed" else "open"

        # Write task file
        task_path = tasks_dir / status_dir / f"{bt_id}.md"
        task_path.write_text(to_markdown(task))
        print(f"Created: {task_path}")

    # Write ID mapping for reference
    mapping_path = output_dir / "id-mapping.json"
    with open(mapping_path, "w") as f:
        json.dump(id_map, f, indent=2)
    print(f"\nID mapping saved to: {mapping_path}")
    print(f"Imported {len(beads)} tasks")


if __name__ == "__main__":
    main()
