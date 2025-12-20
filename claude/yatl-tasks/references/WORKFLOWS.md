# yatl Workflows

Step-by-step workflows with checklists for common scenarios.

## Session Start

### When Starting a New Session

Run this checklist when starting a session in a project with `.tasks/`:

```
Session Start Checklist:
- [ ] yatl next                           # Get suggested task (highest priority ready)
- [ ] yatl list --status in-progress      # Check active work
- [ ] yatl context <id>                   # Full context for active/suggested task
- [ ] Report to user what's available
- [ ] Ask user what to work on (or suggest based on priority)
```

### Example Session Start

```bash
$ yatl next
a1b2  high  Implement JWT authentication
    Users need secure authentication for the API

$ yatl list --status in-progress
e5f6  high  Fix login bug with special characters

$ yatl context e5f6
=== Task ===
ID: e5f6
Title: Fix login bug with special characters
Status: in-progress
Priority: high

=== Recent Activity ===
  2025-01-15 10:30 brian
    Found root cause in auth.py - password escaping issue
```

**Report to user:**
> "Suggested next task: JWT authentication (high priority). There's also 'Fix login bug' in progress - last session found the root cause in auth.py. Should I continue with the login bug fix?"

---

## Session Handoff {#session-handoff}

### When Ending a Session or Approaching Compaction

```
Session End Checklist:
- [ ] Add log entry with current state
- [ ] Include COMPLETED/IN_PROGRESS/NEXT format
- [ ] Document any KEY DECISIONS made
- [ ] Note any BLOCKERS discovered
- [ ] Leave task in appropriate status (in-progress if continuing)
```

### Log Entry Template

```bash
yatl log <task-id> "COMPLETED: [specific deliverables]
IN PROGRESS: [current state]
KEY DECISIONS: [why, not just what]
BLOCKERS: [what's preventing progress, if any]
NEXT: [immediate next step]"
```

### Example Session End

```bash
yatl log a1b2 "COMPLETED: JWT token generation and validation.
KEY DECISION: Using RS256 for asymmetric signing - enables key rotation.
IN PROGRESS: Refresh token implementation.
NEXT: Add token refresh endpoint, then rate limiting."
```

---

## Compaction Survival {#compaction-survival}

### When Compaction is Imminent

**Triggers:**
- User says "running out of context"
- Token usage >70%
- System warning about approaching limits

```
Pre-Compaction Checklist:
- [ ] Identify all active work (yatl list --status in-progress)
- [ ] Add detailed log entry to each active task
- [ ] Verify logs are self-explanatory without conversation context
- [ ] Any discovered work captured as new tasks
- [ ] Ready for fresh session to resume
```

### Post-Compaction Recovery

When starting fresh after compaction:

```
Post-Compaction Checklist:
- [ ] yatl list --status in-progress      # Find where we left off
- [ ] yatl show <task-id>                 # Read log for context
- [ ] Reconstruct understanding from log entries
- [ ] Create TodoWrite items for immediate next steps
- [ ] Continue work
```

### Quality Check for Log Entries

Before finalizing, ask yourself:

1. **Future-me test**: "Could I resume in 2 weeks with zero conversation history?"
2. **Stranger test**: "Could another developer understand this without asking me?"

**Pass criteria:**
- [ ] What was completed? (Specific, not "made progress")
- [ ] What's in progress? (Current state + next step)
- [ ] What decisions were made? (Why, not just what)
- [ ] What's blocked? (Specific blockers)

---

## Task Discovery and Creation

### When Discovering New Work

During exploration or implementation, proactively capture new work:

```
Discovery Checklist:
- [ ] Is this a bug, feature, or task?
- [ ] Is it blocking current work or deferrable?
- [ ] Create task with clear title
- [ ] Add context in body if needed
- [ ] Set appropriate priority
- [ ] Add blocker relationship if needed
```

### Example: Bug Discovery Mid-Task

While working on `a1b2` (JWT auth), you discover a bug:

```bash
# Create the bug task
yatl new "Bug: special characters in password cause login failure" \
    --priority high --tags bug,auth

# If it blocks current work:
yatl block a1b2 <new-bug-id>

# If deferrable, just log it:
yatl log a1b2 "Discovered: special chars bug (task xyz). Deferring - not blocking JWT work."
```

---

## Dependency Management

### Setting Up Work with Dependencies

For multi-step work where order matters:

```
Dependency Planning:
1. [ ] Create all tasks first
2. [ ] Identify blocking relationships
3. [ ] Add blockers: yatl block <blocked> <blocker>
4. [ ] Verify with yatl ready (should show only unblocked work)
5. [ ] Work in dependency order
```

### Example: Feature with Prerequisites

```bash
# Create tasks
yatl new "Set up OAuth credentials" --priority high    # -> a1b2
yatl new "Implement authorization flow" --priority high  # -> c3d4
yatl new "Add token refresh" --priority medium          # -> e5f6
yatl new "Write OAuth tests" --priority medium          # -> g7h8

# Set up dependencies
yatl block c3d4 a1b2    # Auth flow blocked by credentials
yatl block e5f6 c3d4    # Token refresh blocked by auth flow
yatl block g7h8 e5f6    # Tests blocked by token refresh

# Verify
yatl ready              # Should only show a1b2
```

### Working Through Dependencies

```bash
# Start with what's ready
yatl ready              # Shows a1b2
yatl start a1b2
# ... do work ...
yatl close a1b2 --reason "OAuth credentials configured"

yatl ready              # Now shows c3d4 (auto-unblocked!)
yatl start c3d4
# ... continue ...
```

---

## Side Quest Handling

### When Discovering Tangential Work

```
Side Quest Decision:
1. [ ] Is this blocking current task?
2. [ ] Is this more urgent than current task?
3. [ ] Can this be deferred?
```

### Pattern A: Blocking Side Quest

Current task cannot proceed without fixing the side quest.

```bash
# Create side quest
yatl new "Fix: database connection timeout" --priority high --tags bug

# Block current work
yatl block <current-task> <side-quest>

# Work on side quest
yatl start <side-quest>
# ... fix it ...
yatl close <side-quest> --reason "Fixed timeout with connection pooling"

# Current task auto-unblocks, continue
yatl start <current-task>
```

### Pattern B: Deferrable Side Quest

Can note it and continue with current work.

```bash
# Create side quest for later
yatl new "Refactor: auth module could use cleanup" --priority low --tags refactor

# Log it in current task
yatl log <current-task> "Discovered potential refactor (task xyz). Not blocking, continuing."

# Continue current work
```

---

## Multi-Session Project Resume

### Returning After Days/Weeks Away

```
Project Resume Checklist:
- [ ] yatl activity -n 20                 # What happened recently?
- [ ] yatl list --status in-progress      # Any abandoned work?
- [ ] yatl next                           # What's suggested?
- [ ] yatl context <id>                   # Full context for target task
- [ ] Discuss with user what to prioritize
```

### Example Resume

```bash
$ yatl activity -n 5
# See recent activity across all tasks

$ yatl list --status in-progress
e5f6 high Fix login bug

$ yatl context e5f6
=== Task ===
ID: e5f6
Title: Fix login bug with special characters
Status: in-progress
Priority: high

Users cannot log in when password contains special chars...

=== Blocked By ===
  (none)

=== Recent Activity ===
  2025-01-15 10:30 brian
    COMPLETED: Found root cause in auth.py
    IN PROGRESS: Implementing fix
    NEXT: Update password validation regex

# Now you know exactly where to resume
```

---

## Integration with TodoWrite

### Pattern: Long-term yatl + Short-term TodoWrite

```
Start of Session:
1. [ ] Read yatl task log for context
2. [ ] Create TodoWrite items for immediate steps
3. [ ] Work through TodoWrite items
4. [ ] At milestones, update yatl log
5. [ ] At session end, TodoWrite disappears, yatl persists
```

### Example

**yatl task (persistent):**
```bash
yatl show a1b2
# Shows log with previous session context
```

**TodoWrite (ephemeral, this session only):**
```
- [ ] Update password validation regex
- [ ] Add tests for special characters
- [ ] Update error messages
```

**As you work:**
```
- [x] Update password validation regex
- [x] Add tests for special characters
- [ ] Update error messages  # <- still working on this
```

**At milestone:**
```bash
yatl log a1b2 "COMPLETED: Password validation regex updated, tests passing.
IN PROGRESS: Updating error messages for clarity.
NEXT: Final review, then close."
```

---

## Quick Reference: Common Scenarios

| Scenario | Commands |
|----------|----------|
| What should I work on? | `yatl next` |
| What can I work on? | `yatl ready` |
| What's in progress? | `yatl list --status in-progress` |
| Full task context | `yatl context <id>` |
| Search tasks | `yatl list --search "query" --body` |
| Filter by tag | `yatl list --tag bug -n 5` |
| Start working | `yatl start <id>` |
| Add progress note | `yatl log <id> "..."` |
| Finish task | `yatl close <id> --reason "..."` |
| Finish multiple | `yatl close id1 id2 id3 --reason "..."` |
| Found a bug | `yatl new "Bug: ..." --priority high --tags bug` |
| Task A needs B first | `yatl block A B` |
| View dependencies | `yatl tree` |
| Recent activity | `yatl activity -n 10` |
| Resume after break | `yatl next` + `yatl context <id>` |
| Batch create tasks | `yatl import tasks.yaml` |
| JSON output | `yatl list --json` or `yatl show <id> --json` |
