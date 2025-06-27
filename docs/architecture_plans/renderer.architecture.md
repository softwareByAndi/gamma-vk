# Renderer Architecture Plan

## Overview
The Renderer provides a high-level abstraction for recording and submitting draw commands to the GPU, managing frame synchronization, and orchestrating the rendering pipeline. It serves as the primary interface for applications to draw content while hiding the complexity of command buffers, synchronization, and frame management.

## Behavior Specification

### Expected Behaviors (Test-First)
1. **Frame Rendering**: Renderer can record and submit commands for a single frame
   - Test: `test_renderer_renders_empty_frame_succeeds`
   - Edge case: Device lost during rendering

2. **Draw Command Recording**: Renderer can record draw commands with bound resources
   - Test: `test_renderer_draws_triangle_with_vertices`
   - Edge case: Drawing without bound pipeline

3. **Pipeline State Management**: Renderer tracks and validates pipeline state
   - Test: `test_renderer_requires_pipeline_before_draw`
   - Recovery: Clear error message indicating missing pipeline

4. **Resource Binding**: Renderer can bind vertex/index buffers and descriptor sets
   - Test: `test_renderer_binds_vertex_buffer_for_drawing`
   - Edge case: Binding incompatible buffer types

5. **Frame Synchronization**: Renderer manages frame-in-flight synchronization
   - Test: `test_renderer_waits_for_previous_frame`
   - Edge case: Timeout during wait

6. **Command Buffer Lifecycle**: Renderer manages command buffer allocation and reuse
   - Test: `test_renderer_reuses_command_buffers`
   - Edge case: Command buffer allocation failure

### Public API Design
```rust
// Key public interfaces that tests will verify
pub struct Renderer {
    // Internal state hidden from users
}

impl Renderer {
    /// Creates a new renderer with default configuration
    pub fn new(context: &Arc<VulkanContext>) -> Result<Self, GammaVkError> {
        // Test first, implement after
    }
    
    /// Begins recording commands for a new frame
    pub fn begin_frame(&mut self) -> Result<FrameContext, GammaVkError> {
        // Returns a frame context for recording commands
    }
    
    /// Submits the current frame for rendering
    pub fn end_frame(&mut self, frame: FrameContext) -> Result<(), GammaVkError> {
        // Submits recorded commands and manages synchronization
    }
}

/// Context for recording commands within a frame
pub struct FrameContext<'a> {
    // Lifetime tied to renderer
}

impl<'a> FrameContext<'a> {
    /// Binds a graphics pipeline for subsequent draw calls
    pub fn bind_pipeline(&mut self, pipeline: &GraphicsPipeline) -> &mut Self {
        // Fluent API for command recording
    }
    
    /// Binds vertex buffer for drawing
    pub fn bind_vertex_buffer<T>(&mut self, buffer: &VertexBuffer<T>) -> &mut Self {
        // Type-safe buffer binding
    }
    
    /// Records a draw command
    pub fn draw(&mut self, vertex_count: u32) -> Result<&mut Self, GammaVkError> {
        // Validates state and records draw
    }
}
```

## Implementation Checklist

### Key Thought
The most effective starting point is to write one specific test that demonstrates the simplest 
useful behavior of what you're building. This follows the project's core principle: "The test
defines the specification."

Why This Works:
1. Forces Clarity: You can't write a test without knowing exactly what you want
2. Prevents Over-Engineering: You only build what's needed to pass the test
3. Provides Immediate Feedback: You know when you're done
4. Natural Evolution: Each test reveals the next needed piece

### Phase 1: Foundation (Priority: High)
**Goal**: Minimal viable renderer that can submit an empty frame

#### 1.1 Core Types
- [ ] Write test for renderer creation with context
- [ ] Define Renderer struct with minimal fields
- [ ] Implement new() with context validation
- [ ] Add RendererError variants to GammaVkError
- [ ] Document basic usage pattern

#### 1.2 Empty Frame Submission
- [ ] Write test for begin_frame/end_frame cycle
- [ ] Create FrameContext type for command recording
- [ ] Implement command buffer allocation
- [ ] Add frame submission with fence synchronization
- [ ] Test RAII cleanup of frame resources

#### 1.3 Command Buffer Management
- [ ] Write test for command buffer lifecycle
- [ ] Implement command buffer pool/reuse
- [ ] Test allocation failure handling
- [ ] Verify automatic cleanup on drop
- [ ] Add debug assertions for state

#### Phase Gate Validation
Before moving to Phase 2, verify:
- [ ] All Phase 1 tests pass
- [ ] Can submit empty frames repeatedly
- [ ] No resource leaks (verified with logging)
- [ ] Error messages are helpful
- [ ] API feels natural to use

### Phase 2: Drawing Integration (Priority: High)
**Goal**: Renderer can draw a triangle using pipelines and buffers

#### 2.1 Pipeline Integration
- [ ] Write test for pipeline binding requirement
- [ ] Implement bind_pipeline on FrameContext
- [ ] Test drawing without pipeline fails clearly
- [ ] Add pipeline state tracking
- [ ] Document pipeline lifetime requirements

#### 2.2 Buffer Binding
- [ ] Write test for vertex buffer binding
- [ ] Implement type-safe bind_vertex_buffer
- [ ] Test incompatible buffer type rejection
- [ ] Add index buffer support
- [ ] Verify buffer lifetime safety

#### 2.3 Draw Commands
- [ ] Write test for basic draw triangle
- [ ] Implement draw() with state validation
- [ ] Test draw parameter validation
- [ ] Add indexed drawing support
- [ ] Profile draw call overhead

#### 2.4 Render Pass Management
- [ ] Write test for render pass lifecycle
- [ ] Implement automatic render pass begin/end
- [ ] Test clear values and attachments
- [ ] Add render area configuration
- [ ] Document render pass assumptions

#### Phase Gate Validation
Before moving to Phase 3, verify:
- [ ] Triangle example renders correctly
- [ ] State validation prevents invalid draws
- [ ] Performance meets expectations
- [ ] Integration with existing modules works

### Phase 3: Multi-Frame Rendering (Priority: Medium)
**Goal**: Support efficient multi-frame rendering with proper synchronization

#### 3.1 Frame Synchronization
- [ ] Write test for frame-in-flight management
- [ ] Implement fence-based synchronization
- [ ] Test concurrent frame limits
- [ ] Add timeout handling
- [ ] Benchmark synchronization overhead

#### 3.2 Resource Management
- [ ] Write test for per-frame resources
- [ ] Implement uniform buffer updates
- [ ] Test descriptor set management
- [ ] Add push constant support
- [ ] Optimize allocation patterns

#### Phase Gate Validation
Before moving to Phase 4, verify:
- [ ] Multiple frames in flight work correctly
- [ ] No tearing or synchronization issues
- [ ] Resource updates are efficient
- [ ] API supports common patterns

### Phase 4: Advanced Features (Priority: Low)
**Goal**: Production-ready features and optimizations

#### 4.1 Render Graph Support
- [ ] Write test for dependency tracking
- [ ] Design render pass dependency API
- [ ] Implement automatic barriers
- [ ] Test complex dependency chains
- [ ] Document render graph patterns

#### 4.2 Performance Optimizations
- [ ] Write performance benchmarks
- [ ] Implement command buffer caching
- [ ] Add draw call batching
- [ ] Profile and optimize hot paths
- [ ] Document performance characteristics

## Technical Considerations

### Integration Analysis
1. **Direct Dependencies**:
   - Uses: VulkanContext (device, queues, allocator), GraphicsPipeline, Buffer types, ShaderModule
   - Used By: Application layer, examples, higher-level rendering systems
   
2. **Indirect Interactions**:
   - Shared Resources: Swapchain images, command pools, descriptor pools
   - Event Flow: Frame presentation events, resize events
   - State Changes: Pipeline state, bound resources, render passes

### Platform Notes
- macOS/MoltenVK: May have different synchronization requirements
- Cross-platform: Verify frame timing on different platforms
- Performance: Platform-specific optimizations for command recording

### Architecture Decision Records
1. **Frame Context Pattern**: Separate frame recording from renderer state
   - **Alternatives Considered**: Direct methods on renderer, command buffer wrapper
   - **Rationale**: Enforces frame boundaries, prevents state leakage between frames
   - **Trade-offs**: Additional type, but clearer API and better safety

2. **Type-Safe Buffer Binding**: Reuse existing typed buffer wrappers
   - **Alternatives Considered**: Generic buffer binding, runtime type checking
   - **Rationale**: Compile-time safety, consistent with project philosophy
   - **Trade-offs**: More methods, but prevents runtime errors

3. **RAII Command Management**: Automatic command buffer lifecycle
   - **Alternatives Considered**: Manual begin/end commands, explicit management
   - **Rationale**: Consistent with project RAII patterns, prevents leaks
   - **Trade-offs**: Less flexibility, but much safer

### Risk Assessment
- **Main Risk**: Synchronization complexity with multiple frames in flight
- **Mitigation**: Start with single frame, add complexity incrementally
- **Early Warning Signs**: Frame timing inconsistencies, visual artifacts

- **Secondary Risk**: State validation overhead impacting performance
- **Mitigation**: Debug-only validation, fast paths for release builds
- **Early Warning Signs**: Draw call benchmarks showing regression

## Definition of Done
- [ ] All tests pass (unit and integration)
- [ ] Public API documented with examples
- [ ] Error conditions return appropriate GammaVkError variants
- [ ] Resources cleaned up automatically (RAII verified)
- [ ] No clippy warnings
- [ ] Benchmarks show acceptable performance
- [ ] Architecture decisions documented in session_logs/
- [ ] Any discovered gotchas added to debug/ folder
- [ ] Triangle example successfully uses renderer
- [ ] Frame synchronization verified stable

## Pre-Planning Validation Checklist
Before implementing this plan:
- [x] Context gathering completed (existing modules reviewed)
- [x] All phases have clear goals and validation gates
- [x] Cross-cutting concerns addressed (thread safety, errors, performance, memory)
- [x] Integration points identified (VulkanContext, Pipeline, Buffers)
- [x] Architecture decisions documented with rationale
- [x] Plan follows TDD principles from docs/
- [x] Behaviors defined before implementation details
- [x] Each phase delivers working functionality