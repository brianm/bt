# yatl

Yet Another Task List - a minimal, file-based task tracker that lives in your git repo.

## Why?

- **Single binary**: Written in Rust, no runtime dependencies
- **Human readable**: Tasks are markdown files you can edit directly
- **Git native**: Tasks merge, branch, and diff like code
- **Agent friendly**: Structured format for programmatic access
- **Dependency tracking**: Know what's ready to work on

## Quick Start

```bash
# Install with homebrew
brew install brianm/tools/yatl

# Or build and "install" from checked out source
cargo build --release
cp target/release/yatl ~/.bin/ # or wherever

# Initialize in your project
cd your-project
yatl init

# Create a task
yatl new "Fix login bug" --priority high --tags bug,auth

# List tasks
yatl list

# Start working on it
yatl start a1b2

# See what's ready to work on
yatl ready

# Close a task
yatl close a1b2 --reason "Fixed in commit abc123"
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
| `yatl init` | Initialize task tracking |
| `yatl new <title>` | Create a new task |
| `yatl list` | List active tasks (open, in-progress, blocked) |
| `yatl list --all` | List all tasks including closed |
| `yatl show <id>` | Show task details |
| `yatl edit <id>` | Edit task in $EDITOR |
| `yatl start <id>` | Start working (open → in-progress) |
| `yatl stop <id>` | Stop working (in-progress → open) |
| `yatl close <id>` | Close a task |
| `yatl reopen <id>` | Reopen a closed task |
| `yatl ready` | List tasks ready to work on (no blockers) |
| `yatl log <id> <msg>` | Add a log entry |
| `yatl block <id> <by>` | Add a blocker dependency |

### ID Matching

Task IDs use a short hash format like `a1b2c3d4`. You can reference tasks by:
- Full ID: `a1b2c3d4`
- Prefix: `a1b2` or `a1`

## Dependency Tracking

When you block a task (`yatl block A B`), task A is automatically moved to `blocked/`. When task B is closed, task A is automatically moved back to `open/`.

```bash
# Create two tasks
yatl new "Build feature"    # a1b2c3d4
yatl new "Write tests"      # c3d4e5f6

# Block tests on feature
yatl block c3d4 a1b2
# → c3d4e5f6 moves to blocked/

# Close the feature
yatl close a1b2
# → c3d4e5f6 automatically moves back to open/
```

## Workflow with Git

Tasks live in your branch and merge when your code merges:

```bash
# On your feature branch
yatl new "Implement OAuth"
yatl start a1b2
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
yatl ready

# Create a task
yatl new "Fix the bug I found"

# Add a log entry
yatl log a1b2 "Investigated, root cause is in auth.py"

# Close when done
yatl close a1b2 --reason "Fixed in this commit"
```

Or agents can read/write the files directly - the format is simple YAML + markdown. Status is determined by which directory the file is in.

## Thanks

I cribbed (stole) mightily from [beads](https://github.com/steveyegge/beads) for the AI integration. Beads is way more mature, probably better developed, certainly has a larger community, and is probably all around better if you are focused on the task-tracker-for-AI parts. I like `yatl` though, because it is designed for me :-)

## Building

```bash
cargo build --release
```

## License

Apache-2.0
