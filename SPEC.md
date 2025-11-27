# bt Specification

Version: 0.1.0

## Overview

A minimal, file-based task tracking system designed for use with git repositories. Tasks are stored as individual markdown files with YAML frontmatter, enabling both human editing and programmatic access.

## Directory Structure

```
.tasks/
  config.yaml              # Optional: project-level configuration
  .gitattributes           # Git merge strategy configuration
  open/                    # Ready to work on
    bt-a1b2.md
  in-progress/             # Currently being worked on
    bt-c3d4.md
  blocked/                 # Waiting on dependencies
    bt-e5f6.md
  closed/                  # Completed successfully
    bt-f7g8.md
  cancelled/               # Will not be done
    bt-h9i0.md
```

**Status is determined by directory location**, not stored in the file. Tasks move between directories when their status changes. This makes `find .tasks/open -name '*.md'` a trivial way to list active work.

## File Naming Convention

```
{id}.md
```

Where `{id}` is a short hash identifier like `bt-a1b2` (prefix "bt-" followed by 4 hex characters).

IDs are generated from a BLAKE3 hash of:
- Creation timestamp
- Task title
- Random bytes

This ensures uniqueness even with concurrent task creation.

Examples:
- `bt-a1b2.md`
- `bt-c3d4.md`

## Task File Format

```markdown
---
title: Fix login bug with special characters
id: bt-a1b2
created: 2025-11-25T10:30:45Z
updated: 2025-11-25T14:22:00Z
author: brian
priority: high
tags:
  - bug
  - auth
blocked_by: []
---

Users cannot log in when their password contains special characters like `&` or `<`.

## Reproduction

1. Create account with password `test&pass`
2. Log out
3. Attempt to log in
4. Observe 500 error

## Acceptance Criteria

- [ ] Special characters in passwords work correctly
- [ ] Add test coverage for edge cases

---
## Log

Append-only section for updates. Each entry is timestamped.

### 2025-11-25T11:00:00Z brian

Found the root cause - password is being interpolated into SQL without proper escaping in the legacy auth path.

### 2025-11-25T14:22:00Z brian

Fixed in commit abc1234. Need to add tests before closing.
```

## Field Definitions

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `title` | string | Human-readable title |
| `id` | string | Unique identifier (bt-XXXX format) |
| `created` | ISO 8601 | Creation timestamp in UTC |

**Note**: Status is NOT stored in the file. It is derived from the directory the file is in.

### Optional Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `updated` | ISO 8601 | `created` | Last modification timestamp |
| `author` | string | - | Creator's identifier |
| `priority` | enum | `medium` | One of: `low`, `medium`, `high`, `critical` |
| `tags` | list | `[]` | Freeform labels |
| `blocked_by` | list | `[]` | Task IDs that must close before this can proceed |

## Status Transitions

Status is determined by which directory the task file is in:

```
     ┌──────────────────────────────────────┐
     │                                      │
     v                                      │
   open ──────> in-progress ──────> closed  │
     │              │                       │
     │              v                       │
     └────────> blocked ────────────────────┘
     │
     └──────────────────────────> cancelled
```

- `open/`: Ready to be worked on
- `in-progress/`: Currently being worked on
- `blocked/`: Cannot proceed until dependencies resolve
- `closed/`: Completed successfully
- `cancelled/`: Will not be done

### Automatic Status Changes

When you add a blocker to a task (`bt block A B`), task A is automatically moved to `blocked/`.

When a blocking task is closed, all tasks it was blocking are checked - if they have no remaining blockers, they are automatically moved back to `open/`.

## Log Section

The log section follows the YAML frontmatter and main description. It's separated by a horizontal rule (`---`) and a `## Log` heading.

Each log entry:
- Starts with `### {ISO-8601-timestamp} {author}`
- Contains freeform markdown
- Is append-only (never edit previous entries)

This structure makes concurrent additions merge cleanly.

## Dependencies

### blocked_by

List of task IDs that must be `closed` or `cancelled` before this task can proceed.

```yaml
blocked_by:
  - bt-a1b2
  - bt-c3d4
```

When you add a blocker using `bt block`, the task is automatically moved to the `blocked/` directory.

### Determining "Ready" Tasks

A task is ready when:
1. It is in the `open/` directory
2. All tasks in `blocked_by` have been closed or cancelled

## Configuration File

Optional `.tasks/config.yaml`:

```yaml
# Default author for new tasks (falls back to git config user.name)
default_author: brian
```

## Git Integration

### .gitattributes

Created automatically by `bt init`:

```gitattributes
*.md merge=union
```

The `merge=union` strategy concatenates both sides for text conflicts, which works well for the append-only log section. Frontmatter conflicts still need manual resolution, but they're small and obvious.

### Commit Message Convention

```
bt: short description

bt(close): fix login bug
bt(new): add oauth support
bt(update): reprioritize auth work
```

## Agent Integration

Agents can work with tasks by:

1. **CLI**: Use `bt` commands directly
2. **Reading**: Parse YAML frontmatter + markdown body
3. **Creating**: Generate file with proper naming and format
4. **Updating**: Modify frontmatter fields, append to log
5. **Querying**: Use `find`, `grep`, or parse all files

### Example: Find Ready Tasks (pseudocode)

```
for each file in .tasks/open/*.md:
    parse frontmatter
    if blocked_by is empty:
        yield task
    else:
        for dep_id in blocked_by:
            if task_status(dep_id) not in [closed, cancelled]:
                break
        else:
            yield task
```

Note: Tasks in `blocked/` are not considered ready. The automatic blocking system moves tasks to `blocked/` when blockers are added and back to `open/` when all blockers are resolved.

## Comparison with Alternatives

| Feature | bt | Beads | git-bug |
|---------|-----------|-------|---------|
| Storage | Markdown files | JSONL | Git objects |
| Status | Directory-based | Field-based | Field-based |
| Dependencies | Yes (blocked_by) | Yes (4 types) | No |
| Merge handling | Git default + union | Custom driver | Lamport timestamps |
| Query | File parsing | SQL (SQLite) | Custom index |
| Setup | Install binary | Install binary | Install binary |
| Human editable | Yes | No | No |
| Agent friendly | Yes | Yes | Needs CLI |

## Future Considerations

Things explicitly not in v0.1 that could be added:

- **Templates**: `.tasks/templates/bug.md`, etc.
- **Time tracking**: Log entries with duration metadata
- **Kanban board generation**: Static HTML from task data
- **SQLite cache**: For faster queries on large task sets
- **Git hooks**: Automatic syncing
