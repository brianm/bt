# bt vs TodoWrite: Decision Boundaries

Detailed decision criteria for when to use bt versus TodoWrite.

## The Core Distinction

**bt** = Persistent memory that survives session boundaries
**TodoWrite** = Ephemeral checklist for current session only

Think of it as:
- bt = Your project's task backlog
- TodoWrite = Your notepad for the next hour

---

## Decision Framework

### Quick Decision Tree

```
Is this work for THIS session only?
├── Yes → TodoWrite
└── No → Will I need context after compaction?
    ├── Yes → bt
    └── Maybe → bt (better safe than sorry)
```

### Detailed Criteria

| Factor | Use bt | Use TodoWrite |
|--------|--------|---------------|
| **Duration** | Multi-session, days/weeks | Single session, hours |
| **Dependencies** | Has blockers or blocks other work | Linear, no branching |
| **Complexity** | Fuzzy boundaries, exploration | Clear steps, well-defined |
| **Context needed** | Would be hard to resume cold | Can pick up from quick skim |
| **Collaboration** | Others might need to see it | Just for me right now |

---

## Examples by Scenario

### Use bt

**Multi-session projects:**
```bash
bt new "Implement OAuth authentication"
# This will take multiple sessions, has many parts,
# and you'll need context about decisions made
```

**Work with dependencies:**
```bash
bt new "Write tests for auth module"
bt new "Refactor auth after tests pass"
bt block refactor-id tests-id
# The ordering matters and must persist
```

**Knowledge work:**
```bash
bt new "Research caching strategies for API"
# Fuzzy boundaries, discovery process,
# findings need to persist
```

**Bug tracking:**
```bash
bt new "Bug: login fails with special chars" --tags bug
# Even if quick to fix, it's a record for the project
```

### Use TodoWrite

**Single-session implementation:**
```
- [ ] Update the config file
- [ ] Add the new endpoint
- [ ] Test it works
- [ ] Commit the changes
```
All done in one session, linear, no complexity.

**Immediate checklist:**
```
- [ ] Run the tests
- [ ] Fix any failures
- [ ] Push the branch
```
Just need to track steps for the next 30 minutes.

**Simple research:**
```
- [ ] Read the API docs
- [ ] Note the required fields
- [ ] Check rate limits
```
Quick lookup, no persistent context needed.

---

## The Compaction Test

Ask yourself: **"What happens when compaction occurs?"**

### If using TodoWrite:
- TodoWrite list is gone
- Conversation history is gone
- No way to resume without user re-explaining

### If using bt:
- Task file persists in `.tasks/`
- Log entries preserve context
- Can resume by reading `bt show <id>`

**Rule**: If losing context would be painful, use bt.

---

## The Two-Week Test

Ask: **"Could I resume this work in 2 weeks with zero conversation history?"**

### Fails the test (needs bt):
- "Implementing the auth system" - Too many decisions to remember
- "Researching database options" - Findings need to persist
- "Fixing intermittent bug" - Investigation progress matters

### Passes the test (TodoWrite is fine):
- "Update the version number" - Trivial, no context needed
- "Run the deployment script" - Clear steps, no decisions
- "Format the code" - Mechanical, no memory needed

---

## When Both Make Sense

Often you'll use both together:

**bt for the strategic objective:**
```bash
bt new "Implement user authentication"
```

**TodoWrite for tactical execution:**
```
- [x] Create user model
- [x] Add password hashing
- [ ] Implement login endpoint
- [ ] Add session handling
```

**Pattern:**
1. Read bt task for context
2. Create TodoWrite for immediate steps
3. Work through TodoWrite
4. Update bt log with meaningful progress
5. TodoWrite disappears, bt persists

---

## Integration Patterns

### Pattern 1: bt Task Spawns TodoWrite

Start of session:
```bash
bt show a1b2
# Log says: "NEXT: Implement rate limiting"
```

Create TodoWrite:
```
- [ ] Add rate limit middleware
- [ ] Configure limits per endpoint
- [ ] Add tests
- [ ] Update docs
```

Work through TodoWrite, then update bt:
```bash
bt log a1b2 "COMPLETED: Rate limiting with 100 req/min per IP.
Added middleware, tests passing, docs updated."
```

### Pattern 2: TodoWrite Graduates to bt

Start with TodoWrite for quick task:
```
- [ ] Fix the typo in config
- [ ] Update the constant value
```

Discover it's more complex:
```
- [ ] Fix the typo in config
- [x] Update the constant value  # done
- [ ] But wait, this affects 3 other files...
- [ ] And there's a test that needs updating...
```

Graduate to bt:
```bash
bt new "Config value change has broader impact" --tags refactor
bt log <id> "Started as simple change. Discovered affects:
- api/handler.go
- internal/config/defaults.go
- tests/integration_test.go
NEXT: Update all references, verify tests."
```

### Pattern 3: bt for Record, TodoWrite for Speed

Some work deserves a bt record even if done in one session:

```bash
# Create bt for the record
bt new "Security: Update JWT secret rotation" --priority high --tags security

# Use TodoWrite for speed
- [ ] Generate new secrets
- [ ] Update production config
- [ ] Rotate staging
- [ ] Verify all services

# Close with summary for project history
bt close <id> --reason "Rotated JWT secrets across all environments. New 30-day rotation policy."
```

The bt record persists for audit trail even though work was fast.

---

## Common Mistakes

### Mistake 1: Using TodoWrite for Multi-Session Work

**Wrong:**
```
- [ ] Implement OAuth  # This will take days!
```

**Right:**
```bash
bt new "Implement OAuth authentication"
```

### Mistake 2: Using bt for Trivial Tasks

**Wrong:**
```bash
bt new "Update the README typo"  # Overkill
```

**Right:**
Just do it, or use TodoWrite if you need a reminder.

### Mistake 3: Not Updating bt Logs

**Wrong:**
```bash
bt start a1b2
# Work for 2 hours
bt close a1b2
# No log entries - context lost!
```

**Right:**
```bash
bt start a1b2
# Work for 30 min
bt log a1b2 "Implemented token validation"
# Work more
bt log a1b2 "Added refresh logic"
# Finish
bt close a1b2 --reason "Complete with tests"
```

### Mistake 4: Duplicating Between bt and TodoWrite

**Wrong:**
```bash
bt new "Add login endpoint"
# AND
- [ ] Add login endpoint  # Redundant!
```

**Right:**
Use bt for the objective, TodoWrite for sub-steps:
```bash
bt new "Add login endpoint"
# In TodoWrite:
- [ ] Create route handler
- [ ] Add validation
- [ ] Write tests
```

---

## Quick Reference

| Situation | Tool |
|-----------|------|
| "I'll finish this in 10 minutes" | TodoWrite |
| "This might take a few days" | bt |
| "I need to remember why I made this decision" | bt |
| "Just need to not forget these 3 steps" | TodoWrite |
| "Someone else might need to pick this up" | bt |
| "This is blocking other work" | bt |
| "Quick checklist before I push" | TodoWrite |
| "Recording a bug for later" | bt |
| "Running a sequence of commands" | TodoWrite |
| "Exploring options, might pause and resume" | bt |
