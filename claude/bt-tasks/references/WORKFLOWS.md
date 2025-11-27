# bt Workflows

Step-by-step workflows with checklists for common scenarios.

## Session Start

### When Starting a New Session

Run this checklist when starting a session in a project with `.tasks/`:

```
Session Start Checklist:
- [ ] bt next                           # Get suggested task (highest priority ready)
- [ ] bt list --status in-progress      # Check active work
- [ ] bt context <id>                   # Full context for active/suggested task
- [ ] Report to user what's available
- [ ] Ask user what to work on (or suggest based on priority)
```

### Example Session Start

```bash
$ bt next
a1b2  high  Implement JWT authentication
    Users need secure authentication for the API

$ bt list --status in-progress
e5f6  high  Fix login bug with special characters

$ bt context e5f6
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
bt log <task-id> "COMPLETED: [specific deliverables]
IN PROGRESS: [current state]
KEY DECISIONS: [why, not just what]
BLOCKERS: [what's preventing progress, if any]
NEXT: [immediate next step]"
```

### Example Session End

```bash
bt log a1b2 "COMPLETED: JWT token generation and validation.
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
- [ ] Identify all active work (bt list --status in-progress)
- [ ] Add detailed log entry to each active task
- [ ] Verify logs are self-explanatory without conversation context
- [ ] Any discovered work captured as new tasks
- [ ] Ready for fresh session to resume
```

### Post-Compaction Recovery

When starting fresh after compaction:

```
Post-Compaction Checklist:
- [ ] bt list --status in-progress      # Find where we left off
- [ ] bt show <task-id>                 # Read log for context
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
bt new "Bug: special characters in password cause login failure" \
    --priority high --tags bug,auth

# If it blocks current work:
bt block a1b2 <new-bug-id>

# If deferrable, just log it:
bt log a1b2 "Discovered: special chars bug (task xyz). Deferring - not blocking JWT work."
```

---

## Dependency Management

### Setting Up Work with Dependencies

For multi-step work where order matters:

```
Dependency Planning:
1. [ ] Create all tasks first
2. [ ] Identify blocking relationships
3. [ ] Add blockers: bt block <blocked> <blocker>
4. [ ] Verify with bt ready (should show only unblocked work)
5. [ ] Work in dependency order
```

### Example: Feature with Prerequisites

```bash
# Create tasks
bt new "Set up OAuth credentials" --priority high    # -> a1b2
bt new "Implement authorization flow" --priority high  # -> c3d4
bt new "Add token refresh" --priority medium          # -> e5f6
bt new "Write OAuth tests" --priority medium          # -> g7h8

# Set up dependencies
bt block c3d4 a1b2    # Auth flow blocked by credentials
bt block e5f6 c3d4    # Token refresh blocked by auth flow
bt block g7h8 e5f6    # Tests blocked by token refresh

# Verify
bt ready              # Should only show a1b2
```

### Working Through Dependencies

```bash
# Start with what's ready
bt ready              # Shows a1b2
bt start a1b2
# ... do work ...
bt close a1b2 --reason "OAuth credentials configured"

bt ready              # Now shows c3d4 (auto-unblocked!)
bt start c3d4
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
bt new "Fix: database connection timeout" --priority high --tags bug

# Block current work
bt block <current-task> <side-quest>

# Work on side quest
bt start <side-quest>
# ... fix it ...
bt close <side-quest> --reason "Fixed timeout with connection pooling"

# Current task auto-unblocks, continue
bt start <current-task>
```

### Pattern B: Deferrable Side Quest

Can note it and continue with current work.

```bash
# Create side quest for later
bt new "Refactor: auth module could use cleanup" --priority low --tags refactor

# Log it in current task
bt log <current-task> "Discovered potential refactor (task xyz). Not blocking, continuing."

# Continue current work
```

---

## Multi-Session Project Resume

### Returning After Days/Weeks Away

```
Project Resume Checklist:
- [ ] bt activity -n 20                 # What happened recently?
- [ ] bt list --status in-progress      # Any abandoned work?
- [ ] bt next                           # What's suggested?
- [ ] bt context <id>                   # Full context for target task
- [ ] Discuss with user what to prioritize
```

### Example Resume

```bash
$ bt activity -n 5
# See recent activity across all tasks

$ bt list --status in-progress
e5f6 high Fix login bug

$ bt context e5f6
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

### Pattern: Long-term bt + Short-term TodoWrite

```
Start of Session:
1. [ ] Read bt task log for context
2. [ ] Create TodoWrite items for immediate steps
3. [ ] Work through TodoWrite items
4. [ ] At milestones, update bt log
5. [ ] At session end, TodoWrite disappears, bt persists
```

### Example

**bt task (persistent):**
```bash
bt show a1b2
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
bt log a1b2 "COMPLETED: Password validation regex updated, tests passing.
IN PROGRESS: Updating error messages for clarity.
NEXT: Final review, then close."
```

---

## Quick Reference: Common Scenarios

| Scenario | Commands |
|----------|----------|
| What should I work on? | `bt next` |
| What can I work on? | `bt ready` |
| What's in progress? | `bt list --status in-progress` |
| Full task context | `bt context <id>` |
| Search tasks | `bt list --search "query" --body` |
| Filter by tag | `bt list --tag bug -n 5` |
| Start working | `bt start <id>` |
| Add progress note | `bt log <id> "..."` |
| Finish task | `bt close <id> --reason "..."` |
| Finish multiple | `bt close id1 id2 id3 --reason "..."` |
| Found a bug | `bt new "Bug: ..." --priority high --tags bug` |
| Task A needs B first | `bt block A B` |
| View dependencies | `bt tree` |
| Recent activity | `bt activity -n 10` |
| Resume after break | `bt next` + `bt context <id>` |
| Batch create tasks | `bt import tasks.yaml` |
| JSON output | `bt list --json` or `bt show <id> --json` |
