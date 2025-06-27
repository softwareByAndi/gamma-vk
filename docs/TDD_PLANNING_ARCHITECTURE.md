# TDD Planning Architecture

## Overview
This workflow guides the development of architecture plans that prioritize Test-Driven Development (TDD) principles. The goal is to design features where tests define behavior before implementation begins.

## When to Use This Workflow
- Planning new features or modules
- Refactoring existing functionality
- Designing complex integrations
- Before starting any significant development work

## Workflow Steps

### Step 1: Understand the Problem Domain
Before creating an architecture plan:
1. Research similar features in the codebase
2. Check `debug/` notes for relevant gotchas
3. Review Vulkan/graphics requirements
4. Identify user-facing behaviors (not implementation details)

### Step 2: Define Behaviors Through Tests
Start with **what** the feature should do, not **how**:
```rust
// Example: Planning a texture system
// First, write the test that defines behavior:
#[test]
fn texture_loads_from_valid_file() {
    let texture = Texture::from_file(&context, "assets/test.png")?;
    assert_eq!(texture.dimensions(), (256, 256));
    assert_eq!(texture.format(), Format::R8G8B8A8_SRGB);
}
```

### Step 3: Create Architecture Plan
Use the command: `architecture_plan.md {feature_name}`

Focus on:
1. **Test-first thinking**: What tests prove this works?
2. **Incremental delivery**: What's the smallest useful piece?
3. **RAII patterns**: How do resources clean up automatically?
4. **Error handling**: What can fail and how do we recover?

### Step 4: Validate the Plan
Before implementation:
- [ ] Can each phase be tested independently?
- [ ] Does the API feel natural to use?
- [ ] Are error cases well-defined?
- [ ] Is the plan incremental (each phase adds value)?

### Step 5: Implement Using TDD Cycle
For each checklist item:
1. **Red**: Write failing test
2. **Green**: Minimal code to pass
3. **Refactor**: Improve without breaking tests

## Example: Planning Buffer Pool Feature

### 1. Start with Behavior
"I need a way to efficiently reuse GPU buffers"

### 2. Define Through Tests
```rust
#[test]
fn buffer_pool_reuses_same_size_buffers() {
    let pool = BufferPool::new(&context);
    let buffer1 = pool.acquire::<Vertex>(1024)?;
    let id1 = buffer1.id();
    drop(buffer1); // Return to pool
    
    let buffer2 = pool.acquire::<Vertex>(1024)?;
    assert_eq!(buffer2.id(), id1); // Same buffer reused
}
```

### 3. Plan Architecture
- Phase 1: Basic acquire/release with size matching
- Phase 2: Type safety for different buffer types  
- Phase 3: Automatic cleanup of unused buffers
- Phase 4: Performance optimizations

### 4. Implement Incrementally
Each phase has working, tested code before moving to the next.

## Common Pitfalls to Avoid

### 1. Implementation-First Thinking
❌ "I'll use a HashMap to store buffers by size"  
✅ "Buffers of the same size should be reused"

### 2. Big Bang Development
❌ Plan entire system before any tests  
✅ Plan Phase 1, implement, test, then plan Phase 2

### 3. Untestable Designs
❌ Complex initialization with many dependencies  
✅ Simple constructors with dependency injection

### 4. Skipping Error Cases
❌ Only planning happy path  
✅ Define behavior for allocation failures, invalid inputs

## Integration with Project Workflow

1. **Create plan**: Use `architecture_plan.md` command
2. **Save to**: `plans/{feature}.plan.md`
3. **Update TODO.md**: Add Phase 1 tasks
4. **Begin implementation**: Start with first test
5. **Track progress**: Check off completed items
6. **Document insights**: Update session logs

## Benefits of TDD Architecture Planning

- **Clear specifications**: Tests define exact behavior
- **Incremental progress**: Always have working code
- **Design feedback**: Bad designs are hard to test
- **Documentation**: Tests serve as usage examples
- **Confidence**: Refactoring is safe with good tests

## Remember

> "The test is the specification. Write the test first, and you've defined what success looks like."

Good architecture plans make it easy to write tests. If a design is hard to test, it's probably too complex.