# bt CLI Reference

Complete command reference for bt (Brian's Tasks).

## Command Overview

| Command | Description |
|---------|-------------|
| `bt init` | Initialize task tracking in current directory |
| `bt new` | Create a new task |
| `bt import` | Batch create tasks from YAML file |
| `bt list` / `bt ls` | List tasks with filtering |
| `bt show` | Display task details |
| `bt context` | Show full context for working on a task |
| `bt next` | Suggest highest priority ready task |
| `bt activity` | Show recent activity across all tasks |
| `bt tree` | Show dependency tree of active tasks |
| `bt edit` | Edit task in $EDITOR |
| `bt start` | Begin work on task(s) (open -> in-progress) |
| `bt stop` | Pause work on task(s) (in-progress -> open) |
| `bt close` | Complete task(s) (-> closed) |
| `bt reopen` | Revive closed task(s) (closed -> open) |
| `bt ready` | List tasks ready to work on (no blockers) |
| `bt log` | Add entry to task log |
| `bt block` | Add blocker dependency |
| `bt unblock` | Remove blocker dependency |
| `bt update` | Programmatic field updates |

---

## bt init

Initialize task tracking in current directory.

```bash
bt init
```

Creates:
- `.tasks/` directory structure
- `.tasks/config.yaml` - default configuration
- `.tasks/.gitattributes` - git merge strategy (union merge for logs)
- Status directories: `open/`, `in-progress/`, `blocked/`, `closed/`, `cancelled/`

---

## bt new

Create a new task.

```bash
bt new "Task title"
bt new "Task title" [OPTIONS]
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
bt new "Implement OAuth"

# With priority and tags
bt new "Fix login bug" --priority high --tags bug,auth

# With blocker
bt new "Write tests" --blocked-by a1b2

# With piped description (body)
echo "Users cannot log in with special chars in password" | bt new "Fix login bug"
```

**Output:** Task ID and file path

---

## bt import

Batch create tasks from a YAML file with dependencies.

```bash
bt import <file>
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

## bt list / bt ls

List tasks with optional filtering.

```bash
bt list [OPTIONS]
bt ls [OPTIONS]
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
bt list

# All tasks including closed
bt list --all

# Only open tasks
bt list --status open

# High priority tasks
bt list --priority high

# Filter by tag
bt list --tag bug

# Search in title and body
bt list --search "authentication"

# Limit output size
bt list -n 5

# Show body preview
bt list --body

# JSON output for programmatic parsing
bt list --json

# Combined: search bugs, limit to 3, show body
bt list --tag bug --search "login" -n 3 --body

# Verbose format
bt list --long
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

## bt show

Display task details.

```bash
bt show <task-id> [OPTIONS]
```

**Arguments:**
- `task-id` - Full ID or unique prefix (case-insensitive)

**Options:**

| Flag | Description |
|------|-------------|
| `--json` | Output as JSON (machine-readable) |

**Examples:**

```bash
bt show a1b2           # Prefix matching
bt show a1b2c3d4       # Full ID
bt show a1b2 --json    # JSON output for parsing
```

**Output:** Full markdown including frontmatter, body, and log section

---

## bt context

Show full context for working on a task. Combines task details with dependency information.

```bash
bt context <task-id>
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

## bt next

Suggest the highest priority ready task.

```bash
bt next
```

**Algorithm:**
1. Get all ready tasks (no active blockers)
2. Sort by priority (critical > high > medium > low)
3. Within same priority, sort by created date (oldest first)
4. Output the top task with body preview

**Example:**
```bash
$ bt next
a1b2  high  Implement JWT authentication
    Users need secure authentication for the API
```

**Use case:** Decide what to work on without analyzing `bt ready` output.

---

## bt activity

Show recent activity across all tasks.

```bash
bt activity [OPTIONS]
```

**Options:**

| Flag | Short | Description |
|------|-------|-------------|
| `--limit` | `-n` | Maximum entries to show (default: 10) |
| `--all` | `-a` | Include closed/cancelled tasks |

**Examples:**

```bash
bt activity            # Last 10 entries from active tasks
bt activity -n 5       # Last 5 entries
bt activity --all      # Include closed tasks
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

## bt tree

Show dependency tree of active tasks.

```bash
bt tree
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

## bt edit

Edit task in $EDITOR.

```bash
bt edit <task-id>
```

Opens the task markdown file in your configured editor. Changes to title, body, tags, priority are saved. Status changes should be done via `bt start`, `bt stop`, `bt close`.

---

## bt start

Begin work on one or more tasks.

```bash
bt start <task-id> [task-id...]
```

- Moves tasks from `open/` to `in-progress/`
- Adds log entry: "Started working."
- Only works on tasks in `open/` status

**Examples:**
```bash
bt start a1b2              # Start one task
bt start a1b2 c3d4 e5f6    # Start multiple tasks
```

---

## bt stop

Pause work on one or more tasks.

```bash
bt stop <task-id> [task-id...]
```

- Moves tasks from `in-progress/` to `open/`
- Only works on tasks in `in-progress/` status

**Examples:**
```bash
bt stop a1b2               # Stop one task
bt stop a1b2 c3d4          # Stop multiple tasks
```

---

## bt close

Complete one or more tasks.

```bash
bt close <task-id> [task-id...] [OPTIONS]
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
bt close a1b2
bt close a1b2 --reason "Fixed in commit abc123"
bt close a1b2 c3d4 e5f6                         # Close multiple
bt close a1b2 c3d4 --reason "Sprint complete"   # Multiple with reason
```

---

## bt reopen

Revive one or more closed tasks.

```bash
bt reopen <task-id> [task-id...]
```

- Moves tasks from `closed/` to `open/`

**Examples:**
```bash
bt reopen a1b2              # Reopen one task
bt reopen a1b2 c3d4 e5f6    # Reopen multiple tasks
```

---

## bt ready

List tasks ready to work on.

```bash
bt ready
```

Shows only tasks that are:
- In `open/` directory
- Have NO unresolved blockers (blocked_by list is empty or all blockers are closed)

This is the primary command for finding what to work on next.

---

## bt log

Add entry to task log.

```bash
bt log <task-id> "message"
bt log <task-id> "line 1" "line 2" "line 3"
```

- Appends entry to task's `## Log` section
- Automatically timestamped with author
- Multiple arguments are joined with spaces

**Examples:**

```bash
bt log a1b2 "Found root cause in auth.py"
bt log a1b2 "COMPLETED: JWT validation" "NEXT: Add tests"
```

**Log format in file:**
```markdown
### 2025-01-15T10:30:00Z brian

Found root cause in auth.py
```

---

## bt block

Add blocker dependency.

```bash
bt block <task-to-block> <blocker-task>
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
bt block c3d4 a1b2
```

---

## bt unblock

Remove blocker dependency.

```bash
bt unblock <task-id> <blocker-id>
```

**Effects:**
- Removes `blocker-id` from `blocked_by` list
- If no more blockers remain, moves task from `blocked/` to `open/`

---

## bt update

Programmatic field updates.

```bash
bt update <task-id> [OPTIONS]
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
bt update a1b2 --title "New title"
bt update a1b2 --priority critical
bt update a1b2 --tags bug,urgent,auth
bt update a1b2 --add-tag documentation
bt update a1b2 --remove-tag old-tag

# Read body from stdin
cat description.txt | bt update a1b2 --body -
```

---

## Task ID Matching

bt supports prefix matching for task IDs:

- IDs are 8-character Crockford base32 (e.g., `a1b2c3d4`)
- Prefix matching is case-insensitive
- Use shortest unique prefix for convenience

**Examples:**
```bash
bt show a1b2           # Matches a1b2c3d4
bt show A1B2           # Case-insensitive
bt show a1b2c3d4       # Full ID always works
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
| `EDITOR` | Editor for `bt edit` command |

---

## Configuration

Optional `.tasks/config.yaml`:

```yaml
default_author: brian
```

Falls back to `git config user.name` if not set.
