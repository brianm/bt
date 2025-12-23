# yatl CLI Reference

Complete command reference for yatl (Yet Another Task List).

## Command Overview

| Command | Description |
|---------|-------------|
| `yatl init` | Initialize task tracking in current directory |
| `yatl new` | Create a new task |
| `yatl import` | Batch create tasks from YAML file |
| `yatl list` / `yatl ls` | List tasks with filtering |
| `yatl show` | Display task details |
| `yatl context` | Show full context for working on a task |
| `yatl next` | Suggest highest priority ready task |
| `yatl activity` | Show recent activity across all tasks |
| `yatl tree` | Show dependency tree of active tasks |
| `yatl edit` | Edit task in $EDITOR |
| `yatl start` | Begin work on task(s) (open -> in-progress) |
| `yatl stop` | Pause work on task(s) (in-progress -> open) |
| `yatl close` | Complete task(s) (-> closed) |
| `yatl reopen` | Revive closed task(s) (closed -> open) |
| `yatl ready` | List tasks ready to work on (no blockers) |
| `yatl log` | Add entry to task log |
| `yatl block` | Add blocker dependency |
| `yatl unblock` | Remove blocker dependency |
| `yatl update` | Programmatic field updates |

---

## yatl init

Initialize task tracking in current directory.

```bash
yatl init
```

Creates:
- `.tasks/` directory structure
- `.tasks/config.yaml` - default configuration
- `.tasks/.gitattributes` - git merge strategy (union merge for logs)
- Status directories: `open/`, `in-progress/`, `blocked/`, `closed/`, `cancelled/`

---

## yatl new

Create a new task.

```bash
yatl new "Task title"
yatl new "Task title" [OPTIONS]
```

**Options:**

| Flag | Short | Description |
|------|-------|-------------|
| `--priority` | `-p` | Priority: low, medium (default), high, critical |
| `--tags` | `-t` | Comma-separated tags |
| `--blocked-by` | `-b` | Comma-separated task IDs that block this task |

**Examples:**

```bash
# Simple task
yatl new "Implement OAuth"

# With priority and tags
yatl new "Fix login bug" --priority high --tags bug,auth

# With blocker
yatl new "Write tests" --blocked-by a1b2

# With piped description (body)
echo "Users cannot log in with special chars in password" | yatl new "Fix login bug"
```

**Output:** Task ID and file path

---

## yatl import

Batch create tasks from a YAML file with dependencies.

```bash
yatl import <file>
```

**YAML format:**

```yaml
- alias: setup          # Local reference name
  title: "Set up database"
  priority: high
  tags: [backend, db]
  body: "Configure PostgreSQL connection"

- alias: api
  title: "Build API endpoints"
  priority: medium
  tags: [backend]
  blocked_by: [setup]   # Reference by alias
  body: "REST endpoints for user management"

- alias: tests
  title: "Write API tests"
  blocked_by: [api]
  body: "Integration tests for all endpoints"
```

**Key features:**
- `alias` - Local name for referencing within the file
- `blocked_by` - Can reference aliases or existing task IDs
- Tasks created in order, dependencies resolved automatically
- Tasks with blockers automatically placed in `blocked/`

**Use case:** Planning multi-step work with dependencies upfront.

---

## yatl list / yatl ls

List tasks with optional filtering.

```bash
yatl list [OPTIONS]
yatl ls [OPTIONS]
```

**Options:**

| Flag | Short | Description |
|------|-------|-------------|
| `--all` | `-a` | Include closed/cancelled tasks |
| `--long` | `-l` | Verbose output format |
| `--status` | `-s` | Filter by status: open, in-progress, blocked, closed, cancelled |
| `--priority` | `-p` | Filter by priority: low, medium, high, critical |
| `--tag` | `-t` | Filter by tag |
| `--search` | | Search in title and body |
| `--limit` | `-n` | Limit number of results |
| `--body` | `-b` | Show body preview (first line) |
| `--json` | | Output as JSON (machine-readable) |

**Examples:**

```bash
# Active tasks (open, in-progress, blocked)
yatl list

# All tasks including closed
yatl list --all

# Only open tasks
yatl list --status open

# High priority tasks
yatl list --priority high

# Filter by tag
yatl list --tag bug

# Search in title and body
yatl list --search "authentication"

# Limit output size
yatl list -n 5

# Show body preview
yatl list --body

# JSON output for programmatic parsing
yatl list --json

# Combined: search bugs, limit to 3, show body
yatl list --tag bug --search "login" -n 3 --body

# Verbose format
yatl list --long
```

**Output:** ID (shortest unique prefix), status, priority, title

**JSON output structure:**
```json
[
  {
    "id": "a1b2c3d4",
    "short_id": "a1b2",
    "title": "Fix login bug",
    "status": "open",
    "priority": "high",
    "tags": ["bug", "auth"],
    "blocked_by": [],
    "created": "2025-01-15T10:30:00Z",
    "updated": "2025-01-15T14:22:00Z",
    "author": "brian",
    "body": "Users cannot log in..."
  }
]
```

---

## yatl show

Display task details.

```bash
yatl show <task-id> [OPTIONS]
```

**Arguments:**
- `task-id` - Full ID or unique prefix (case-insensitive)

**Options:**

| Flag | Description |
|------|-------------|
| `--json` | Output as JSON (machine-readable) |

**Examples:**

```bash
yatl show a1b2           # Prefix matching
yatl show a1b2c3d4       # Full ID
yatl show a1b2 --json    # JSON output for parsing
```

**Output:** Full markdown including frontmatter, body, and log section

---

## yatl context

Show full context for working on a task. Combines task details with dependency information.

```bash
yatl context <task-id>
```

**Output includes:**
- Full task details (title, status, priority, tags, body)
- List of blocking tasks with their status
- List of tasks this blocks
- Recent log entries

**Example output:**
```
=== Task ===

ID: a1b2
Title: Implement JWT authentication
Status: blocked
Priority: high
Tags: auth, backend

Users need secure authentication...

=== Blocked By ===

  c3d4 Set up OAuth credentials [open]

=== Blocks ===

  e5f6 Write auth tests [blocked]

=== Recent Activity ===

  2025-01-15 10:30 brian
    Found good JWT library - using jsonwebtoken
```

**Use case:** Load full context before starting work on a task.

---

## yatl next

Suggest the highest priority ready task.

```bash
yatl next
```

**Algorithm:**
1. Get all ready tasks (no active blockers)
2. Sort by priority (critical > high > medium > low)
3. Within same priority, sort by created date (oldest first)
4. Output the top task with body preview

**Example:**
```bash
$ yatl next
a1b2  high  Implement JWT authentication
    Users need secure authentication for the API
```

**Use case:** Decide what to work on without analyzing `yatl ready` output.

---

## yatl activity

Show recent activity across all tasks.

```bash
yatl activity [OPTIONS]
```

**Options:**

| Flag | Short | Description |
|------|-------|-------------|
| `--limit` | `-n` | Maximum entries to show (default: 10) |
| `--all` | `-a` | Include closed/cancelled tasks |

**Examples:**

```bash
yatl activity            # Last 10 entries from active tasks
yatl activity -n 5       # Last 5 entries
yatl activity --all      # Include closed tasks
```

**Output:** Chronological log entries from all tasks, most recent first.

```
2025-01-15 14:30  a1b2  Implement JWT auth
    brian Started working.
2025-01-15 14:22  c3d4  Fix login bug
    brian Closed: Fixed in commit abc123
2025-01-15 10:30  a1b2  Implement JWT auth
    brian Found good JWT library
```

**Use case:** See what's been happening across all tasks.

---

## yatl tree

Show dependency tree of active tasks.

```bash
yatl tree
```

Displays a visual DAG (Directed Acyclic Graph) of task dependencies:
- Green IDs = ready (no blockers)
- Red IDs = blocked
- Shows "(blocked by: x, y)" for tasks with multiple blockers

**Example:**
```
a1b2  Set up OAuth credentials
├── c3d4  Implement auth flow
│   └── e5f6  Add token refresh
│       └── g7h8  Write auth tests
└── x9y0  Configure CORS (blocked by: a1b2, z1w2)
```

**Use case:** Visualize work dependencies and find the critical path.

---

## yatl edit

Edit task in $EDITOR.

```bash
yatl edit <task-id>
```

Opens the task markdown file in your configured editor. Changes to title, body, tags, priority are saved. Status changes should be done via `yatl start`, `yatl stop`, `yatl close`.

---

## yatl start

Begin work on one or more tasks.

```bash
yatl start <task-id> [task-id...]
```

- Moves tasks from `open/` to `in-progress/`
- Adds log entry: "Started working."
- Only works on tasks in `open/` status

**Examples:**
```bash
yatl start a1b2              # Start one task
yatl start a1b2 c3d4 e5f6    # Start multiple tasks
```

---

## yatl stop

Pause work on one or more tasks.

```bash
yatl stop <task-id> [task-id...]
```

- Moves tasks from `in-progress/` to `open/`
- Only works on tasks in `in-progress/` status

**Examples:**
```bash
yatl stop a1b2               # Stop one task
yatl stop a1b2 c3d4          # Stop multiple tasks
```

---

## yatl close

Complete one or more tasks.

```bash
yatl close <task-id> [task-id...] [OPTIONS]
```

**Options:**

| Flag | Short | Description |
|------|-------|-------------|
| `--reason` | `-r` | Closing reason (added to log, applies to all) |

**Effects:**
- Moves tasks to `closed/`
- Adds log entry with optional reason
- **Automatically unblocks** any tasks that were blocked by this one

**Examples:**

```bash
yatl close a1b2
yatl close a1b2 --reason "Fixed in commit abc123"
yatl close a1b2 c3d4 e5f6                         # Close multiple
yatl close a1b2 c3d4 --reason "Sprint complete"   # Multiple with reason
```

---

## yatl reopen

Revive one or more closed tasks.

```bash
yatl reopen <task-id> [task-id...]
```

- Moves tasks from `closed/` to `open/`

**Examples:**
```bash
yatl reopen a1b2              # Reopen one task
yatl reopen a1b2 c3d4 e5f6    # Reopen multiple tasks
```

---

## yatl ready

List tasks ready to work on.

```bash
yatl ready
```

Shows only tasks that are:
- In `open/` directory
- Have NO unresolved blockers (blocked_by list is empty or all blockers are closed)

This is the primary command for finding what to work on next.

---

## yatl log

Add entry to task log.

```bash
yatl log <task-id> "message"
yatl log <task-id> "line 1" "line 2" "line 3"
```

- Appends entry to task's `## Log` section
- Automatically timestamped with author
- Multiple arguments are joined with spaces

**Examples:**

```bash
yatl log a1b2 "Found root cause in auth.py"
yatl log a1b2 "COMPLETED: JWT validation" "NEXT: Add tests"
```

**Log format in file:**
```markdown
### 2025-01-15T10:30:00Z brian

Found root cause in auth.py
```

---

## yatl block

Add blocker dependency.

```bash
yatl block <task-to-block> <blocker-task>
```

**Arguments:**
- `task-to-block` - Task that will be blocked
- `blocker-task` - Task that blocks it

**Effects:**
- Adds `blocker-task` to `blocked_by` list
- Moves `task-to-block` to `blocked/` directory (if not already)
- Adds log entry: "Added blocker: {blocker-id}"

**Example:**

```bash
# c3d4 (tests) is blocked by a1b2 (implementation)
yatl block c3d4 a1b2
```

---

## yatl unblock

Remove blocker dependency.

```bash
yatl unblock <task-id> <blocker-id>
```

**Effects:**
- Removes `blocker-id` from `blocked_by` list
- If no more blockers remain, moves task from `blocked/` to `open/`

---

## yatl update

Programmatic field updates.

```bash
yatl update <task-id> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `--title` | Replace title |
| `--priority` | Change priority (low, medium, high, critical) |
| `--tags` | Replace all tags (comma-separated) |
| `--add-tag` | Add single tag |
| `--remove-tag` | Remove single tag |
| `--body` | Update description (use `-` for stdin) |

**Examples:**

```bash
yatl update a1b2 --title "New title"
yatl update a1b2 --priority critical
yatl update a1b2 --tags bug,urgent,auth
yatl update a1b2 --add-tag documentation
yatl update a1b2 --remove-tag old-tag

# Read body from stdin
cat description.txt | yatl update a1b2 --body -
```

---

## Task ID Matching

yatl supports prefix matching for task IDs:

- IDs are 8-character Crockford base32 (e.g., `a1b2c3d4`)
- Prefix matching is case-insensitive
- Use shortest unique prefix for convenience

**Examples:**
```bash
yatl show a1b2           # Matches a1b2c3d4
yatl show A1B2           # Case-insensitive
yatl show a1b2c3d4       # Full ID always works
```

---

## Status Directories

Tasks are organized by status in directories:

| Directory | Status | Description |
|-----------|--------|-------------|
| `open/` | Ready | Can be worked on |
| `in-progress/` | Active | Currently being worked on |
| `blocked/` | Waiting | Has unresolved blockers |
| `closed/` | Done | Completed successfully |
| `cancelled/` | Dropped | Will not be done |

Status changes move files between directories. This is the source of truth for task status.

---

## Priority Levels

| Priority | Use Case |
|----------|----------|
| `critical` | Urgent, drop everything |
| `high` | Important, do soon |
| `medium` | Normal priority (default) |
| `low` | Nice to have, do when convenient |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `EDITOR` | Editor for `yatl edit` command |

---

## Configuration

Optional `.tasks/config.yaml`:

```yaml
default_author: brian
```

Falls back to `git config user.name` if not set.
