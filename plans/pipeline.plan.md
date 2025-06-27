# Pipeline Architecture Plan

## Overview
The pipeline system provides type-safe, RAII-managed abstractions for Vulkan graphics and compute pipelines. It enables efficient GPU program configuration through a builder pattern API while preventing common state configuration errors at compile time.

## Behavior Specification

### Expected Behaviors (Test-First)

1. **Pipeline Creation**: Successfully create graphics pipeline with minimal configuration
   - Test: `test_graphics_pipeline_creates_with_minimal_config`
   - Edge case: Missing required shader stages should fail with clear error

2. **Shader Stage Validation**: Validate shader compatibility during pipeline creation
   - Test: `test_pipeline_fails_with_incompatible_shader_stages`
   - Recovery: Return specific error indicating shader mismatch

3. **Render Pass Compatibility**: Ensure pipeline is compatible with target render pass
   - Test: `test_pipeline_validates_render_pass_compatibility`
   - Edge case: Attachment format mismatches should fail early

4. **State Configuration**: Builder pattern prevents invalid state combinations
   - Test: `test_pipeline_builder_validates_required_fields`
   - Edge case: Building without vertex shader returns descriptive error

5. **RAII Cleanup**: Pipeline resources automatically cleaned up on drop
   - Test: `test_pipeline_drop_releases_gpu_resources`
   - Verification: Use debug markers to verify GPU cleanup

6. **Pipeline Caching**: Reuse compiled pipelines when possible
   - Test: `test_pipeline_cache_reduces_compilation_time`
   - Edge case: Cache invalidation on shader modification

### Public API Design
```rust
// Key public interfaces that tests will verify
pub struct GraphicsPipeline {
    inner: Arc<vulkano::pipeline::GraphicsPipeline>,
    layout: Arc<PipelineLayout>,
}

impl GraphicsPipeline {
    pub fn builder() -> GraphicsPipelineBuilder {
        GraphicsPipelineBuilder::new()
    }
    
    pub fn bind(&self, command_buffer: &mut CommandBuffer) -> Result<(), GammaVkError> {
        // Test first, implement after
    }
}

pub struct GraphicsPipelineBuilder {
    vertex_shader: Option<Arc<ShaderModule>>,
    fragment_shader: Option<Arc<ShaderModule>>,
    vertex_input: Option<VertexInputDescription>,
    render_pass: Option<Arc<RenderPass>>,
    // ... other configuration
}

impl GraphicsPipelineBuilder {
    pub fn vertex_shader(mut self, shader: &Arc<ShaderModule>) -> Self {
        self.vertex_shader = Some(Arc::clone(shader));
        self
    }
    
    pub fn build(self, context: &Arc<VulkanContext>) -> Result<GraphicsPipeline, GammaVkError> {
        // Validate all required fields present
        // Create Vulkan pipeline
        // Wrap in RAII type
    }
}
```

## Implementation Checklist

### Phase 1: Foundation (Priority: High)
**Goal**: Minimal viable graphics pipeline for triangle rendering

#### 1.1 Core Types and Error Handling
- [ ] Write test for `PipelineError` enum variants
- [ ] Define `PipelineError` in error.rs with specific variants:
  - `ShaderStageMissing(stage_name)`
  - `ShaderIncompatible(reason)`
  - `RenderPassIncompatible(reason)`
  - `VertexInputMismatch(expected, actual)`
- [ ] Write test for basic `GraphicsPipeline` type creation
- [ ] Define `GraphicsPipeline` struct with Arc<vulkano::pipeline::GraphicsPipeline>
- [ ] Implement Debug and Display traits
- [ ] Document RAII behavior and thread safety

#### 1.2 Builder Pattern Implementation
- [ ] Write test for builder with minimal configuration
- [ ] Create `GraphicsPipelineBuilder` struct
- [ ] Write test for vertex shader configuration
- [ ] Implement `vertex_shader()` method
- [ ] Write test for fragment shader configuration
- [ ] Implement `fragment_shader()` method
- [ ] Write test for missing required fields error
- [ ] Implement validation in `build()` method
- [ ] Document builder pattern usage

#### 1.3 Basic Pipeline Creation
- [ ] Write integration test with existing ShaderModule
- [ ] Implement pipeline creation using Vulkano API
- [ ] Write test for vertex input configuration
- [ ] Add vertex input state to builder
- [ ] Write test for render pass specification
- [ ] Add render pass to builder
- [ ] Verify RAII cleanup with drop test
- [ ] Document pipeline state requirements

#### Phase Gate Validation
Before moving to Phase 2, verify:
- [ ] All Phase 1 tests pass
- [ ] Can create pipeline for triangle rendering
- [ ] Error messages are descriptive and actionable
- [ ] API follows &Arc<T> pattern consistently
- [ ] No regression in existing tests

### Phase 2: Integration (Priority: Medium)
**Goal**: Pipeline system works with full rendering pipeline

#### 2.1 State Configuration
- [ ] Write test for viewport and scissor configuration
- [ ] Add dynamic state specification
- [ ] Write test for depth/stencil configuration
- [ ] Implement depth/stencil state
- [ ] Write test for blending configuration
- [ ] Add color blending options
- [ ] Test primitive topology options
- [ ] Document state interactions

#### 2.2 Pipeline Layout and Descriptors
- [ ] Write test for descriptor set layout creation
- [ ] Implement `PipelineLayout` type
- [ ] Write test for push constant configuration
- [ ] Add push constant support to builder
- [ ] Write integration test with UniformBuffer
- [ ] Implement descriptor binding helpers
- [ ] Test multiple descriptor sets
- [ ] Document resource binding patterns

#### 2.3 Command Buffer Integration
- [ ] Write test for pipeline binding in command buffer
- [ ] Implement `bind()` method
- [ ] Write test for draw command validation
- [ ] Add pipeline state tracking
- [ ] Test pipeline switching overhead
- [ ] Implement state change optimization
- [ ] Document command buffer usage

#### Phase Gate Validation
Before moving to Phase 3, verify:
- [ ] Integration tests with full render loop pass
- [ ] Pipeline state changes are efficient
- [ ] Descriptor binding is intuitive
- [ ] Thread safety verified with concurrent tests
- [ ] API documentation complete with examples

### Phase 3: Performance (Priority: Low)
**Goal**: Optimize pipeline creation and switching

#### 3.1 Pipeline Caching
- [ ] Write benchmark for pipeline compilation time
- [ ] Design pipeline cache interface
- [ ] Write test for cache hit/miss behavior
- [ ] Implement pipeline cache with Vulkano
- [ ] Test cache serialization/deserialization
- [ ] Add cache persistence options
- [ ] Benchmark cache effectiveness
- [ ] Document caching strategy

#### 3.2 Compute Pipeline Support
- [ ] Write test for compute pipeline creation
- [ ] Create `ComputePipeline` type
- [ ] Write test for workgroup configuration
- [ ] Implement compute-specific builder
- [ ] Test dispatch command integration
- [ ] Add compute shader validation
- [ ] Document compute patterns

#### 3.3 Advanced Features
- [ ] Write test for pipeline derivatives
- [ ] Implement derivative pipeline creation
- [ ] Test dynamic pipeline state
- [ ] Add tessellation shader support
- [ ] Test geometry shader pipeline
- [ ] Implement pipeline statistics
- [ ] Document advanced usage

## Technical Considerations

### Integration Analysis
1. **Direct Dependencies**:
   - Uses: `VulkanContext` (device access), `ShaderModule` (shader stages), `RenderPass` (compatibility)
   - Used By: Command buffer recording, render loop, material system (future)
   
2. **Indirect Interactions**:
   - Shared Resources: Descriptor sets, vertex buffers, uniform buffers
   - Event Flow: Pipeline bind events affect subsequent draw calls
   - State Changes: Pipeline switches can be expensive, needs optimization

### Platform Notes
- macOS/MoltenVK: Some pipeline features may not be supported (geometry shaders, tessellation)
- Cross-platform: Verify dynamic state support varies by driver
- Validation layers: Essential for catching pipeline configuration errors early

### Architecture Decision Records
1. **Separate Pipeline Types**: `GraphicsPipeline` vs `ComputePipeline`
   - **Alternatives Considered**: Generic `Pipeline` type with enum discrimination
   - **Rationale**: Type safety prevents mixing graphics/compute operations
   - **Trade-offs**: Some code duplication for RAII implementation

2. **Builder Pattern**: Mandatory for pipeline creation
   - **Alternatives Considered**: Direct constructor with many parameters
   - **Rationale**: Pipelines have 15+ configuration options, builder provides clarity
   - **Trade-offs**: Additional API surface, but improves usability

3. **Arc-based Sharing**: Pipelines wrapped in Arc for sharing
   - **Alternatives Considered**: Lifetime-based borrowing
   - **Rationale**: Pipelines often shared between render passes and threads
   - **Trade-offs**: Reference counting overhead minimal compared to pipeline cost

### Risk Assessment
- **Main Risk**: Pipeline state explosion with many permutations
- **Mitigation**: Pipeline cache and derivative pipelines for variations
- **Early Warning Signs**: Excessive pipeline compilation time, memory usage

## Definition of Done
- [ ] All tests pass (unit and integration)
- [ ] Graphics pipeline can render test triangle
- [ ] Public API documented with examples
- [ ] Error conditions return appropriate PipelineError variants
- [ ] Pipeline resources cleaned up automatically (RAII verified)
- [ ] No clippy warnings
- [ ] Benchmarks show acceptable compilation time
- [ ] Architecture decisions documented in session_logs/
- [ ] Vulkano API gotchas added to debug/vulkano_api.debug.md
- [ ] Platform-specific issues documented in debug/