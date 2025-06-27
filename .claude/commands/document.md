# Document Session Workflow

## Purpose
Document all work completed in the current session, update project state, and maintain knowledge base.

## Workflow Steps

### 1. Update TODO.md
- Mark completed tasks as DONE with completion date
- Move completed items to appropriate sections
- Update progress percentages for current iteration
- Add any new tasks discovered during session
- Adjust priority levels based on insights gained

### 2. Create/Update Session Log
- File: `session_logs/{iteration}_{task_name}.log.md`
- Include:
  - Session date and primary focus
  - Key accomplishments
  - Technical decisions made
  - Challenges encountered and solutions
  - Code snippets for notable implementations
  - Next steps and recommendations

### 3. Document Debug Insights
- Review session for tricky bugs or API gotchas
- Create/update files in `debug/`:
  - `debug_<module>.md` for module-specific issues
  - `debug_<api_name>_api.md` for external API learnings
  - Include problem description, root cause, and solution
  - Add code examples showing wrong vs right approaches

### 4. Update Plan Files
- Review `plans/*.plan.md` files
- Update:
  - Current project state
  - Completed milestones
  - Revised timelines if needed
  - New technical insights affecting plans
  - Risk assessments based on discoveries

### 5. Update Architecture Docs (if applicable)
- If architectural decisions were made:
  - Update `docs/ARCHITECTURE.md`
  - Document new patterns in relevant docs
  - Update module structure if changed

### 6. Re-index Documentation
```bash
# Re-index all documentation for semantic search
python tools/semantic_search.py index .

# Verify indexing success
python tools/semantic_search.py stats
```

## Template for Session Log

```markdown
# Session Log: {task_name}

**Date**: {date}
**Iteration**: {iteration_number}
**Focus**: {primary_focus}

## Accomplishments
- [ ] Task 1 completed
- [ ] Task 2 completed

## Technical Decisions
### Decision 1: {title}
**Context**: Why this decision was needed
**Choice**: What was decided
**Rationale**: Why this approach was chosen

## Challenges & Solutions
### Challenge 1: {description}
**Issue**: Detailed problem description
**Root Cause**: Why it occurred
**Solution**: How it was resolved
```code rust
// Example code showing solution
```

## Key Code Changes
### {Module/Feature}
```rust
// Notable implementation
```

## Insights & Learnings
- Learning 1: {description}
- Learning 2: {description}

## Next Steps
1. {Next task}
2. {Next task}

## Notes for Future Sessions
- {Important reminder}
- {Architectural consideration}
```

## Checklist
- [ ] TODO.md updated with completed tasks
- [ ] Session log created with all key information
- [ ] Debug notes added for tricky issues
- [ ] Plan files updated if project state changed
- [ ] Any new architectural decisions documented
- [ ] Code examples included for future reference
- [ ] Documentation re-indexed with semantic_search.py (last item)