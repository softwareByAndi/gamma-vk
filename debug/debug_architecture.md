# Architecture Decision Debug Notes

## Device Parameter (2025-06-26)
**Issue**: Thought unused `Arc<Device>` parameter should be removed  
**Reality**: Needed for future device validation and API consistency  
**Lesson**: Don't remove "unused" parameters without understanding architectural purpose

## Device Parameter Strategy (2025-06-26)
**Decision**: Keep `Arc<Device>` in buffer constructors despite not directly used  
**Rationale**: Future device validation, API consistency, resource association  
**Validated**: Staff engineer review confirmed architectural soundness

## RAII Resource Management (2025-06-26)
**Choice**: Rust ownership system for automatic GPU cleanup  
**Pattern**: `Drop` trait implementation, `Subbuffer<[T]>` wrappers  
**Benefit**: Compile-time enforcement prevents memory leaks

## Type Safety Strategy (2025-06-26)
**Pattern**: Separate `VertexBuffer`, `IndexBuffer`, `UniformBuffer` types  
**Goal**: Prevent buffer misuse at compile time  
**Implementation**: Wrapper types around base `Buffer` struct

## Error Handling Approach (2025-06-26)
**Strategy**: Domain-specific error types (`BufferCreation`, `ShaderCompilation`)  
**vs Generic**: Single error type with string messages  
**Benefit**: Structured errors, better context preservation

## API Design Philosophy (2025-06-26)
**Levels**: High-level convenience + mid-level control + low-level escape hatches  
**Example**: `VertexBuffer::new()` → `Buffer::new()` → `buffer.inner()`  
**Goal**: Simple things simple, complex things possible

## Module Organization (2025-06-26)
**Choice**: Functional grouping (buffer.rs, shader.rs) vs type grouping  
**Benefit**: Related functionality stays together, easier maintenance