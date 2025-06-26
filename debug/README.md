# Debug Notes System

## Purpose
Capture development insights, API gotchas, and architectural decisions to prevent repeated mistakes and preserve knowledge.

## Files
- **`debug_buffer.md`** - Buffer implementation lessons
- **`debug_vulkano_api.md`** - Vulkano library patterns  
- **`debug_architecture.md`** - Architecture decision rationale
- **`debug_rust_types.md`** - Rust type system gotchas

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