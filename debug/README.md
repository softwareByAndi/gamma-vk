# Debug Notes System

## Purpose
Capture development insights, API gotchas, and architectural decisions to prevent repeated mistakes and preserve knowledge.

## Files
- **`buffer.debug.md`** - Buffer implementation lessons
- **`vulkano_api.debug.md`** - Vulkano library patterns  
- **`architecture.debug.md`** - Architecture decision rationale
- **`rust_types.debug.md`** - Rust type system gotchas
- `ls -l debug/` - List all debug files

## When to Add Entries
- Wrong assumptions corrected by documentation
- API patterns that differ from expectations  
- Type system issues and resolutions
- Architecture decisions that might seem unclear later

## Entry Format
```markdown
### Issue Title (YYYY-MM-DD)
**Issue**: What went wrong?  
**Reality/Fix**: What was the solution?  
**Lesson**: Key takeaway
```

## Guidelines
- **Be Concise**: Essential insights only
- **Include Context**: Why this matters
- **Reference Code**: File/line when helpful
- **Update Regularly**: Add insights as they occur

The debug notes preserve hard-won knowledge and prevent repeating the same mistakes.