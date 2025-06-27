You are creating a TDD-oriented architecture plan for {$ARGUMENTS} in Gamma-VK, a Vulkan graphics engine in Rust. Think like a senior engineer who prioritizes testable behavior over implementation details.

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

## Context to Consider:

1. **Check existing patterns**:
   - use `python tools/semantic_search.py` to find semantically related features
   - Similar features in the codebase
   - Relevant debug notes in `debug/`
   - Architectural guidelines in `docs/`

2. **Vulkan/Graphics specifics**:
   - Resource lifecycle (RAII patterns)
   - Thread safety requirements
   - Platform differences (especially MoltenVK)

3. **Project principles**:
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

### Phase 2: Integration (Priority: Medium)

#### 2.1 System Integration
- [ ] Write integration test with {existing_module}
- [ ] Implement integration points
- [ ] Test resource sharing
- [ ] Verify thread safety
- [ ] Document integration patterns

### Phase 3: Performance (Priority: Low)

#### 3.1 Optimizations
- [ ] Write performance benchmarks
- [ ] Implement caching/pooling
- [ ] Profile and optimize hot paths
- [ ] Document performance characteristics

## Technical Considerations

### Dependencies
- Depends on: {existing modules}
- Used by: {future modules}

### Platform Notes
- macOS/MoltenVK: {specific considerations}
- Cross-platform: {what to verify}

### Risk Assessment
- **Main Risk**: {what could go wrong}
- **Mitigation**: {how to handle it}

## Definition of Done
- [ ] All tests pass
- [ ] Public API documented with examples
- [ ] Error conditions return appropriate GammaVkError variants
- [ ] Resources cleaned up automatically (RAII)
- [ ] No clippy warnings
- [ ] Benchmarks show acceptable performance
```

Remember: Write the test first. The test defines the specification. Implementation follows to make the test pass.