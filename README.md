# bt

Brian's Tasks - a minimal, file-based task tracker that lives in your git repo.

## Why?

- **Single binary**: Written in Rust, no runtime dependencies
- **Human readable**: Tasks are markdown files you can edit directly
- **Git native**: Tasks merge, branch, and diff like code
- **Agent friendly**: Structured format for programmatic access
- **Dependency tracking**: Know what's ready to work on

## Quick Start

```bash
# Install from source
cargo install --path .

# Initialize in your project
cd your-project
bt init

# Create a task
bt new "Fix login bug" --priority high --tags bug,auth

# List tasks
bt list

# Start working on it
bt start a1b2

# See what's ready to work on
bt ready

# Close a task
bt close bt-a1b2 --reason "Fixed in commit abc123"
```

## Task Format

Tasks are markdown files with YAML frontmatter. **Status is determined by directory location**, not stored in the file:

```
.tasks/
├── open/           # Ready to work on
├── in-progress/    # Currently being worked on
├── blocked/        # Waiting on dependencies
├── closed/         # Completed successfully
└── cancelled/      # Will not be done
```

Example task file:

```markdown
---
title: Fix login bug with special characters
id: a1b2c3d4
created: 2025-11-25T10:30:45Z
updated: 2025-11-25T14:22:00Z
author: brian
priority: high
tags:
  - bug
  - auth
blocked_by: []
---

Users cannot log in when password contains special characters.

---
## Log

### 2025-11-25T11:00:00Z brian

Found the root cause in legacy auth path.
```

## Commands

| Command | Description |
|---------|-------------|
| `bt init` | Initialize task tracking |
| `bt new <title>` | Create a new task |
| `bt list` | List active tasks (open, in-progress, blocked) |
| `bt list --all` | List all tasks including closed |
| `bt show <id>` | Show task details |
| `bt edit <id>` | Edit task in $EDITOR |
| `bt start <id>` | Start working (open → in-progress) |
| `bt stop <id>` | Stop working (in-progress → open) |
| `bt close <id>` | Close a task |
| `bt reopen <id>` | Reopen a closed task |
| `bt ready` | List tasks ready to work on (no blockers) |
| `bt log <id> <msg>` | Add a log entry |
| `bt block <id> <by>` | Add a blocker dependency |

### ID Matching

Task IDs use a short hash format like `a1b2c3d4`. You can reference tasks by:
- Full ID: `a1b2c3d4`
- Prefix: `a1b2` or `a1`

## Dependency Tracking

When you block a task (`bt block A B`), task A is automatically moved to `blocked/`. When task B is closed, task A is automatically moved back to `open/`.

```bash
# Create two tasks
bt new "Build feature"    # bt-a1b2
bt new "Write tests"      # bt-c3d4

# Block tests on feature
bt block c3d4 a1b2
# → bt-c3d4 moves to blocked/

# Close the feature
bt close a1b2
# → bt-c3d4 automatically moves back to open/
```

## Workflow with Git

Tasks live in your branch and merge when your code merges:

```bash
# On your feature branch
bt new "Implement OAuth"
bt start a1b2
# ... work on feature ...
git add .tasks/
git commit -m "feat: oauth support"
git push
# Task becomes visible when PR merges
```

## Agent Usage

Agents can work with tasks directly:

```bash
# List ready tasks
bt ready

# Create a task
bt new "Fix the bug I found"

# Add a log entry
bt log a1b2 "Investigated, root cause is in auth.py"

# Close when done
bt close a1b2 --reason "Fixed in this commit"
```

Or agents can read/write the files directly - the format is simple YAML + markdown. Status is determined by which directory the file is in.

## Thanks

I cribbed (stole) mightily from [beads](https://github.com/steveyegge/beads) for the AI integration. Beads is way more mature, probably better developed, certainly has a larger community, and is probably all around better if you are focused on the task-tracker-for-AI parts. I like `bt` though, because it is designed for me :-)

## Building

```bash
cargo build --release
```

## License

MIT
