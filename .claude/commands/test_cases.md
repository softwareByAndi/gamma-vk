# Role:
You are a master of test driven development. 

## some notes about existing code

- existing code is not guaranteed to be correct, so don't assume it works
- existing code is not guaranteed to be complete, so don't assume it has all the functionality you need
- existing code is not guaranteed to be idiomatic, so don't assume it follows best practices
- existing code is not guaranteed to be performant, so don't assume it is optimized
- existing code is not guaranteed to be secure, so don't assume it is safe

# Directions

1. **Context:** read the `/docs/TESTING_PATTERNS.md` file to understand the testing patterns used in this project.

2. **Read the existing code** in `/src/{$ARGUMENTS}.rs` to understand its purpose and  functionality. Look for any existing tests or comments that might give you insight into the intended behavior.

3. **Goal:** Some code has been written in `/src/`{$ARGUMENTS}.rs, and your job is to test it. But don't just write tests to match the logic. Think deeply about what these tests should accomplish and what role they play in the code-base and development cycle. Consider what use cases and edge cases should be accounted for - whether it's currently implemented or not. the code will be refactored to match the tests you write, so write them well.

## Task List
- document a list of test cases in `/tests/{$ARGUMENTS}.tests.md` that cover the TDD plan for the code from `/src/{$ARGUMENTS}.rs` (even if the code is not yet implemented).
- use `/tests/{$ARGUMENTS}.tests.md` as a scratch pad, marking items off as you implement them. 
- tests should be implemented in `/tests/{$ARGUMENTS}.rs` and should be runnable with `cargo test`
    - don't implement tests for code that is not yet implemented
    - tests should be comprehensive, covering both expected behavior and edge cases
    - tests should be organized and grouped logically, in a way that makes it easy to understand the purpose of each test
- Use the patterns from TESTING_PATTERNS.md as a guide, but also think creatively about how to structure your tests.

