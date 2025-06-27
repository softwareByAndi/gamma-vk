You are creating a TDD-oriented architecture plan for {$ARGUMENTS} in Gamma-VK, a Vulkan graphics engine in Rust. Think like a senior engineer who prioritizes testable behavior over implementation details.

## Step 0: Context Gathering (5 minutes)
Before planning, gather relevant context:
- [ ] Search for similar patterns: `python tools/semantic_search.py search "{feature type} implementation"`
- [ ] Check for existing abstractions: `python tools/semantic_search.py search "trait for {behavior}"`
- [ ] Review error handling patterns: `python tools/semantic_search.py search "error handling {domain}"`
- [ ] Check `debug/` folder for relevant API gotchas and lessons learned
- [ ] Review `docs/` for architectural patterns and principles
- [ ] Look at existing implementations in `src/` for established patterns

## Your Plan Should Address:

### 1. **Behavior Specification Through Tests**
Before any implementation details, define the expected behaviors:
- What tests will verify this feature works correctly?
- What edge cases and error conditions must be handled?
- What integration points need testing?
- How will we know when this feature is complete?

### 2. **Architecture Structure**
Design the architecture to support TDD:
- **Public API**: What will users interact with? (test this first)
- **Core Components**: What internal pieces enable the public API?
- **Dependencies**: What existing modules will this integrate with?
- **Extension Points**: Where might this need to grow in the future?

### 3. **Implementation Phases**
Break down implementation into testable increments:
- **Phase 1**: Minimal viable functionality (what's the simplest useful thing?)
- **Phase 2**: Core feature completeness
- **Phase 3**: Performance and optimization
- **Phase 4**: Advanced features and nice-to-haves

### 4. **TDD Checklist Format**
For each phase, provide checkboxes in this order:
1. [ ] Write test for behavior X
2. [ ] Implement minimal code to pass test
3. [ ] Refactor for clarity/performance
4. [ ] Document public API
5. [ ] Add integration test if needed

### 5. **Cross-Cutting Concerns**
Analyze how this feature handles:
- **Thread Safety**: How will this work across threads?
- **Error Propagation**: How do errors bubble up to users?
- **Performance Impact**: What are the hot paths and bottlenecks?
- **Memory Management**: What owns what? How is cleanup handled?

## Context to Consider:

1. **Reference existing patterns**:
   - Test patterns: See `docs/TESTING_PATTERNS.md` for test naming and structure
   - RAII examples: Check `docs/RAII_PATTERN.md` and existing implementations in `src/`
   - Architecture workflow: Follow `docs/TDD_ARCHITECTURE_WORKFLOW.md`
   - Code style: Refer to `docs/STYLE_GUIDE.md` for conventions

2. **Vulkan/Graphics specifics**:
   - Resource lifecycle (RAII patterns - see `src/buffer.rs` for examples)
   - Thread safety requirements
   - Platform differences (especially MoltenVK - check `src/context.rs`)

3. **Project principles** (from `docs/DESIGN_PRINCIPLES.md`):
   - Safety by Default
   - Performance by Design
   - Extensible by Nature
   - Progressive Disclosure

## Output Format:

```markdown
# {Feature} Architecture Plan

## Overview
{2-3 sentences describing what this feature provides and why it's needed}

## Behavior Specification

### Expected Behaviors (Test-First)
1. **{Behavior 1}**: {What should happen}
   - Test: `test_{behavior}_succeeds_when_{condition}`
   - Edge case: {What could go wrong}

2. **{Behavior 2}**: {What should happen}
   - Test: `test_{behavior}_fails_when_{condition}`
   - Recovery: {How to handle failure}

### Public API Design
```rust
// Key public interfaces that tests will verify
pub struct {Type} {
    // ...
}

impl {Type} {
    pub fn new() -> Result<Self, GammaVkError> {
        // Test first, implement after
    }
}
```

## Implementation Checklist

### Phase 1: Foundation (Priority: High)
**Goal**: Minimal viable functionality that proves the concept

#### 1.1 Core Types
- [ ] Write test for basic type creation
- [ ] Define public struct/trait
- [ ] Implement new() with validation
- [ ] Add error types for failure cases
- [ ] Document behavior and examples

#### 1.2 Basic Operations
- [ ] Write test for primary operation
- [ ] Implement operation method
- [ ] Test error conditions
- [ ] Verify RAII cleanup
- [ ] Add debug/display traits

#### Phase Gate Validation
Before moving to Phase 2, verify:
- [ ] All Phase 1 tests pass
- [ ] No regression in existing tests
- [ ] API feels ergonomic (can you explain it simply?)
- [ ] Error paths are tested and documented

### Phase 2: Integration (Priority: Medium)
**Goal**: Feature works with existing system components

#### 2.1 System Integration
- [ ] Write integration test with {existing_module}
- [ ] Implement integration points
- [ ] Test resource sharing
- [ ] Verify thread safety
- [ ] Document integration patterns

#### Phase Gate Validation
Before moving to Phase 3, verify:
- [ ] Integration tests pass
- [ ] No performance regression
- [ ] Thread safety verified
- [ ] API documentation complete

### Phase 3: Performance (Priority: Low)
**Goal**: Optimize for production use

#### 3.1 Optimizations
- [ ] Write performance benchmarks
- [ ] Implement caching/pooling
- [ ] Profile and optimize hot paths
- [ ] Document performance characteristics

## Technical Considerations

### Integration Analysis
1. **Direct Dependencies**:
   - Uses: {modules this directly depends on}
   - Used By: {modules that will depend on this}
   
2. **Indirect Interactions**:
   - Shared Resources: {what resources are accessed}
   - Event Flow: {what events does this emit/consume}
   - State Changes: {what global state is affected}

### Platform Notes
- macOS/MoltenVK: {specific considerations}
- Cross-platform: {what to verify}

### Architecture Decision Records
1. **Key Decision**: {what approach was chosen}
   - **Alternatives Considered**: {other options evaluated}
   - **Rationale**: {why this approach}
   - **Trade-offs**: {what we're giving up}

### Risk Assessment
- **Main Risk**: {what could go wrong}
- **Mitigation**: {how to handle it}
- **Early Warning Signs**: {how to detect problems}

## Definition of Done
- [ ] All tests pass (unit and integration)
- [ ] Public API documented with examples
- [ ] Error conditions return appropriate GammaVkError variants
- [ ] Resources cleaned up automatically (RAII verified)
- [ ] No clippy warnings
- [ ] Benchmarks show acceptable performance
- [ ] Architecture decisions documented in session_logs/
- [ ] Any discovered gotchas added to debug/ folder
```

## Pre-Planning Validation Checklist
Before submitting this plan:
- [ ] Context gathering completed (Step 0)
- [ ] All phases have clear goals and validation gates
- [ ] Cross-cutting concerns addressed
- [ ] Integration points identified
- [ ] Architecture decisions documented
- [ ] Plan follows TDD principles from docs/

Remember: Write the test first. The test defines the specification. Implementation follows to make the test pass.