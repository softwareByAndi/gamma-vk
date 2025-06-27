# Session 07: Pipeline Architecture Planning

**Date**: 2025-06-27
**Focus**: TDD-oriented architecture planning for Vulkan pipeline abstractions

## Context Gathering

### Existing State Analysis
- No pipeline implementation exists yet
- Prerequisites in place: VulkanContext, ShaderModule, Buffer types
- Error framework ready but no pipeline-specific variants
- Architecture positioned in graphics abstraction layer

### Key Findings from Research

1. **API Patterns**:
   - All public APIs use `&Arc<T>` pattern (not `Arc<T>`)
   - Vulkano requires owned `Arc<Device>` internally
   - Clone only when storing, not on every call

2. **Architectural Principles**:
   - Type-safe distinctions (GraphicsPipeline vs ComputePipeline)
   - RAII for automatic GPU resource cleanup
   - Builder pattern for complex configuration
   - Progressive disclosure with sensible defaults

3. **Platform Considerations**:
   - MoltenVK may not support all pipeline features
   - Validation critical for early error detection
   - Performance implications of pipeline switching

## Architecture Decisions

### 1. Type-Safe Pipeline Types
**Decision**: Separate `GraphicsPipeline` and `ComputePipeline` types

**Rationale**:
- Prevents mixing incompatible operations at compile time
- Clearer API intent
- Follows established buffer type pattern (VertexBuffer, IndexBuffer, etc.)

**Trade-offs**:
- Some RAII code duplication
- Separate builder implementations

### 2. Builder Pattern Requirement
**Decision**: Pipeline creation exclusively through builders

**Rationale**:
- 15+ configuration options make direct construction unwieldy
- Validates configuration before expensive GPU operations
- Enables progressive disclosure of advanced options

**Implementation Strategy**:
```rust
GraphicsPipeline::builder()
    .vertex_shader(&vertex_shader)
    .fragment_shader(&fragment_shader)
    .vertex_input(vertex_description)
    .render_pass(&render_pass)
    .build(&context)?
```

### 3. Shader Integration Design
**Decision**: Builders take `&Arc<ShaderModule>` and clone internally

**Rationale**:
- Consistent with established API patterns
- Avoids forcing users to clone in hot paths
- Shader modules are immutable and shareable

### 4. Error Handling Strategy
**Decision**: Domain-specific `PipelineError` variants

**Planned Variants**:
- `ShaderStageMissing(stage_name)` - Required shader not provided
- `ShaderIncompatible(reason)` - Shader interface mismatch
- `RenderPassIncompatible(reason)` - Pipeline/render pass mismatch
- `VertexInputMismatch(expected, actual)` - Vertex format issues

## TDD Implementation Plan

### Phase 1: Foundation (Current Focus)
1. Error types with tests
2. Basic GraphicsPipeline type with RAII
3. Minimal builder for triangle rendering
4. Integration with existing modules

### Phase 2: Full Pipeline State
1. Dynamic state configuration
2. Pipeline layouts and descriptors
3. Command buffer integration
4. State optimization

### Phase 3: Performance & Advanced
1. Pipeline caching
2. Compute pipeline support
3. Derivative pipelines
4. Advanced shader stages

## Technical Insights

### Vulkano API Considerations
- Pipeline creation needs owned `Arc<Device>`
- Render pass compatibility checked at creation time
- Dynamic state reduces pipeline permutations
- Pipeline layout critical for descriptor binding

### Performance Implications
- Pipeline creation is expensive (compile shaders)
- Pipeline switching has CPU/GPU cost
- Caching essential for real applications
- Derivative pipelines help with variations

## Next Steps

1. Start Phase 1 implementation with error types
2. Create comprehensive test suite following TDD
3. Document Vulkano-specific gotchas discovered
4. Validate against triangle rendering use case

## Risks and Mitigations

**Risk**: Pipeline state explosion
- **Mitigation**: Pipeline cache, derivative pipelines, dynamic state

**Risk**: Platform incompatibilities
- **Mitigation**: Feature detection, graceful degradation, clear errors

**Risk**: Complex API surface
- **Mitigation**: Builder pattern, good defaults, progressive disclosure

## Architecture Validation

The proposed architecture:
- ✓ Follows established project patterns
- ✓ Integrates with existing modules
- ✓ Supports TDD methodology
- ✓ Handles platform differences
- ✓ Scales from simple to complex use cases
- ✓ Maintains safety and performance principles