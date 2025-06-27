# Error Handling Architecture: Next Steps

## Quick Start: Your First TDD Cycle

### 1. Run the Example Test (Red Phase)
```bash
# This will fail - that's expected!
cargo test test_error_context_includes_operation_details -- --ignored

# See the failure and understand what needs to be built
```

### 2. Implement Minimal Code (Green Phase)
Start with the simplest enhancement to `src/error.rs`:
```rust
// Add to GammaVkError
pub struct ErrorContext {
    operation: String,
    details: Vec<(String, String)>,
}

impl GammaVkError {
    pub fn with_context(mut self, operation: impl Into<String>) -> Self {
        // Minimal implementation to make test pass
        todo!()
    }
}
```

### 3. Refactor and Enhance
Once the test passes, improve the implementation while keeping tests green.

## Implementation Priority Order

### Week 1: Foundation
1. **Start Here**: Implement ErrorContext struct (Phase 1.1)
   - [ ] Run example test to see it fail
   - [ ] Add ErrorContext to error.rs
   - [ ] Make test pass with minimal code
   - [ ] Refactor for cleanliness

2. **Recovery Hints** (Phase 1.2)
   - [ ] Write test for recovery hints
   - [ ] Add hint field and methods
   - [ ] Add hints to common errors

3. **Maintain Compatibility** (Phase 1.3)
   - [ ] Ensure all existing tests still pass
   - [ ] Update error creation sites incrementally

### Week 2: Integration
4. **Error Categorization** (Phase 2.1)
   - [ ] Add severity levels
   - [ ] Classify existing errors

5. **Platform Normalization** (Phase 2.2)
   - [ ] Handle MoltenVK quirks
   - [ ] Normalize error messages

### Week 3+: Advanced Features
6. **Error Chains** (Phase 2.3)
7. **Error Collection** (Phase 3.1)
8. **Telemetry** (Phase 3.2)

## Key Files to Modify

1. **src/error.rs** - Main implementation
2. **tests/error_handling_tdd_example.rs** - Your TDD tests
3. **src/context.rs** - Update error creation sites
4. **src/buffer.rs** - Add context to buffer errors
5. **src/shader.rs** - Add context to shader errors

## Success Metrics

- [ ] Error messages are 2x more helpful (subjective but important)
- [ ] Zero breaking changes to existing code
- [ ] Error creation adds < 5 lines of code per site
- [ ] Performance impact < 1% in error paths
- [ ] 100% test coverage for new error features

## Common Pitfalls to Avoid

1. **Don't Start with the Perfect Design**
   - Write one test, make it pass, iterate
   - Perfect is the enemy of good

2. **Don't Break Existing Code**
   - Run `cargo test` frequently
   - Use `#[deprecated]` for gradual migration

3. **Don't Over-Engineer**
   - Start with context and hints
   - Add advanced features only if needed

4. **Don't Forget Documentation**
   - Update examples as you go
   - Document patterns in session_logs/

## Example Migration

### Before:
```rust
Buffer::new(size)
    .map_err(|e| GammaVkError::BufferCreation(e.to_string()))?
```

### After (Phase 1):
```rust
Buffer::new(size)
    .map_err(|e| GammaVkError::buffer_creation(e))
    .context("creating vertex buffer")?
    .with_detail("size", size.to_string())?
```

### After (Phase 2):
```rust
Buffer::new(size)
    .map_err(|e| GammaVkError::buffer_creation(e))
    .context("creating vertex buffer")?
    .with_recovery_hint("Try reducing buffer size or freeing GPU memory")?
```

## Remember: Start Small, Test First

The architecture plan is comprehensive, but you don't need to implement everything at once. Start with one failing test, make it pass, then move to the next. Each small step moves the project forward.

> "The test defines the specification." - Write the test first!