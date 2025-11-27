# Claude Code Skill for bt (Brian's Tasks)

A Claude Code skill that teaches Claude how to use bt effectively for task tracking in multi-session coding workflows.

## What is This?

This is a [Claude Code](https://claude.com/claude-code) skill - a markdown-based instruction set that teaches Claude AI how to use bt. It provides the **philosophy and patterns** of effective bt usage for persistent task tracking.

## What Does It Provide?

**Main skill file (SKILL.md):**
- Core workflow patterns (discovery, execution, planning phases)
- Decision criteria for when to use bt vs TodoWrite
- Session start protocols and ready work checks
- Compaction survival patterns (critical for Claude Code context limits)
- Task lifecycle management with self-check checklists
- Dependency management with automatic blocking/unblocking
- Integration patterns with TodoWrite

**Reference documentation:**
- `references/CLI_REFERENCE.md` - Complete command reference with all flags
- `references/WORKFLOWS.md` - Step-by-step workflows with checklists
- `references/BOUNDARIES.md` - Detailed decision criteria for bt vs TodoWrite

## Prerequisites

1. Install bt (Brian's Tasks) CLI - build from source:
   ```bash
   cd /path/to/bt
   cargo build --release
   # Add target/release/bt to your PATH
   ```

2. Have [Claude Code](https://claude.com/claude-code) installed

3. Initialize bt in your project:
   ```bash
   cd your-project
   bt init
   ```

## Installation

### Option 1: Symlink (Recommended)

```bash
# Clone bt if you haven't already
git clone https://github.com/brianm/bt.git
cd bt

# Create a symlink in your Claude Code skills directory
ln -s "$(pwd)/claude/bt-task-tracking" ~/.claude/skills/bt-task-tracking
```

### Option 2: Copy Files Directly

```bash
# Copy the skill files
cp -r /path/to/bt/claude/bt-task-tracking ~/.claude/skills/
```

## Configure Hooks (Recommended)

Add session start and pre-compaction hooks to `~/.claude/settings.json`:

```json
{
  "hooks": {
    "PreCompact": [
      {
        "matcher": ".tasks",
        "hooks": [
          {
            "type": "command",
            "command": "echo '## bt Task Status' && bt list --status in-progress && echo '' && bt ready"
          }
        ]
      }
    ],
    "SessionStart": [
      {
        "matcher": ".tasks",
        "hooks": [
          {
            "type": "command",
            "command": "echo '## bt Task Status' && bt list --status in-progress && echo '' && bt ready"
          }
        ]
      }
    ]
  }
}
```

This shows in-progress and ready tasks:
- At session start (when `.tasks/` directory exists)
- Before compaction (so task state is visible in compacted context)

## Verify Installation

Restart Claude Code, then in a new session with a project that has `.tasks/`, ask:

```
Do you have the bt skill installed?
```

Claude should confirm it has access to the bt skill and can help with task tracking.

## How It Works

Claude Code automatically loads skills from `~/.claude/skills/`. When this skill is installed:

1. Claude gets the core workflow from `SKILL.md` immediately
2. Claude can read reference docs when it needs detailed information
3. The skill uses progressive disclosure - quick reference in SKILL.md, details in references/

## Usage Examples

Once installed, Claude will automatically:

- Check for ready work at session start (if `.tasks/` exists and hooks configured)
- Suggest creating bt tasks for multi-session work
- Use appropriate dependency patterns when linking tasks
- Maintain proper task lifecycle (create -> start -> close)
- Know when to use bt vs TodoWrite

You can also explicitly ask Claude to use bt:

```
Let's track this work in bt since it spans multiple sessions
```

```
Create a bt task for this bug we discovered
```

```
Show me what's ready to work on
```

## Key Differences from beads

bt is intentionally simpler than beads:

| bt | beads |
|----|-------|
| Plain markdown files | SQLite + JSONL database |
| Status = directory location | Status in database |
| Just use git directly | `bd sync` command |
| `bt new` | `bd create` |
| `bt log` (append to file) | `bd update --notes` |
| `bt start` / `bt stop` | `bd update --status` |
| Priority + tags | Types (epic, bug, feature) + priority |

## License

MIT License
