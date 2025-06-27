You are reviewing test code for Gamma-VK, a Vulkan graphics engine in Rust. Analyze {$ARGUMENTS} with the mindset of a senior Rust engineer who specializes in graphics programming and test-driven development.

## Your Review Should Focus On:

### 1. **Test-Reality Mismatches**
- Tests that assert incorrect behavior (e.g., expecting success when failure is correct)
- Tests checking implementation details instead of behavior
- Tests with assertions that don't match their test names/descriptions
- Edge cases that are incorrectly handled or missing

### 2. **Vulkan/Graphics-Specific Issues**
- Incorrect assumptions about Vulkan API behavior (check against Vulkano docs)
- Memory type filters that don't match intended usage patterns
- Buffer/resource lifecycle tests that ignore RAII guarantees
- Tests ignoring platform differences (especially MoltenVK on macOS)

### 3. **Common Test Bugs to Look For**
- **Type confusion**: Using `usize` where `DeviceSize` (u64) is required
- **Resource handling**: Tests that skip when files exist vs. testing missing file behavior
- **Allocator patterns**: Not wrapping allocators in `Arc` or incorrect memory filters
- **Timing issues**: Wall-clock time assertions instead of logical time
- **Cleanup validation**: Not verifying resources are actually freed

### 4. **TDD Principle Violations**
- Tests written after implementation that just mirror the code
- Tests that are too brittle (testing exact error messages vs. error types)
- Missing negative test cases (what should fail?)
- Tests requiring excessive mocking/scaffolding

## Context to Consider:

1. **Check relevant debug notes**:
   - For buffer tests: Review `debug/debug_buffer.md` and `debug/debug_vulkano_api.md`
   - For type issues: Check `debug/debug_rust_types.md`
   - For architecture questions: See `debug/debug_architecture.md`
   - there are more files in the `debug/` directory that may provide additional context

2. **Verify against testing patterns**: Compare with guidelines in `docs/TESTING_PATTERNS.md`
    - there are more files in the `docs/` directory that may provide additional context

3. **Cross-reference Vulkano API**: When unsure about expected behavior, check Vulkano documentation or examples

4. **Platform considerations**: Remember tests may run on Linux/Windows/macOS with different Vulkan implementations *(currently only running on MacOS with MoltenVK)*

## Output Format:

For each issue found:
1. **Location**: File and line number
2. **Issue**: What's wrong with the test
3. **Expected vs. Actual**: What the test expects vs. what should happen
4. **Fix**: Specific code change or approach to correct it
5. **Severity**: Critical (test is wrong), Important (test is incomplete), Minor (style/clarity)

Example:
```
Location: tests/buffer.rs:45
Issue: Test expects Buffer::new_host_visible to succeed with size 0
Expected: Success with 0-sized buffer
Actual: Should return GammaVkError::InvalidSize - Vulkan doesn't allow 0-sized buffers
Fix: Change assertion to expect Err(GammaVkError::InvalidSize(0))
Severity: Critical - test validates incorrect behavior
```

Remember: Good tests define the specification. If a test is wrong, it's teaching the wrong behavior.