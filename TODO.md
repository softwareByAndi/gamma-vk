# Gamma-VK Development TODO

## Overview
This document tracks the development progress of Gamma-VK using iterative development principles. Each iteration focuses on delivering working, tested functionality while building toward a complete graphics engine.

## Current Status
- **Project Phase**: Foundation/Setup
- **Current Iteration**: Pre-Development (Documentation and Planning)
- **Next Iteration**: Make it a Library (Iteration 1)

## Iteration Strategy
Based on staff engineer reviews, the approach focuses on **working software over perfect process**. Priorities:
- ✅ Local development with real Vulkan (no mocking)
- ✅ Comprehensive unit tests run locally
- ⏸️ CI/CD deferred until complexity is justified
- ⏸️ Cross-platform testing deferred (design for it, test when hardware available)
- 🔄 Error handling improved iteratively, not built perfect upfront

## Getting Started (Your First Day)
1. **Verify baseline**: Run `cargo run` - confirm Hello World works on your system
2. **Start development**: Begin Iteration 1, Day 1, first task
3. **Create branch**: `git checkout -b iteration-1-library`
4. **Begin with**: Configure Cargo.toml with basic dev-dependencies

## Iteration Planning

### Pre-Development Phase ✅
**Goal**: Establish project foundation and development methodology
**Duration**: Complete
**Status**: ✅ COMPLETED

#### Completed Tasks
- [x] Create project structure documentation
- [x] Establish coding style guidelines  
- [x] Define design principles
- [x] Research iterative development practices
- [x] Create CLAUDE.md for future development
- [x] Set up basic project with working Vulkan "Hello World"

---

### Iteration 1: Make it a Library (Planned)
**Goal**: Transform Hello World into a reusable library with basic testing
**Duration**: 1 week
**Status**: 🔄 PLANNED
**Dependencies**: None (foundation work)

#### Must Have (MVP)
- [ ] **Project Structure Setup** (Day 1)
  - [ ] Configure `Cargo.toml` with basic dev-dependencies (thiserror for errors)
  - [ ] Create `src/lib.rs` as main library entry point with `pub use` exports
  - [ ] Move current `main.rs` to `examples/hello_world/main.rs`
  - [ ] Update `Cargo.toml` with `[[example]]` section for hello_world

- [ ] **Basic Error Handling** (Day 2)
  - [ ] Create `src/error.rs` with simple `GammaVkError` enum wrapping `vulkano::VulkanError`
  - [ ] Add `From<vulkano::VulkanError>` conversion using thiserror
  - [ ] Replace `expect()` calls in main.rs with proper `Result<T, GammaVkError>` 
  - [ ] Update hello_world example to handle errors gracefully

- [ ] **VulkanContext Extraction** (Day 3-4)
  - [ ] Create `src/context.rs` with `VulkanContext` struct
  - [ ] Extract Vulkan initialization logic from main.rs into `VulkanContext::new()`
  - [ ] Implement `Drop` for `VulkanContext` with proper cleanup
  - [ ] Handle MoltenVK portability requirements in `VulkanContext`

- [ ] **Testing and Validation** (Day 5)
  - [ ] Set up local quality gates: `cargo clippy`, `cargo fmt`, `cargo test`
  - [ ] Add unit tests for error conversion in `src/error.rs`
  - [ ] Add integration test for `VulkanContext` creation/destruction
  - [ ] Update hello_world example to use new library API

#### Success Criteria (Measurable)
- [ ] `examples/hello_world/main.rs` uses `gamma_vk` library in <30 lines of code
- [ ] `cargo test` passes with basic unit and integration tests
- [ ] `cargo clippy` produces zero warnings on default configuration
- [ ] `VulkanContext::new()` and `Drop` work correctly (no resource leaks)
- [ ] Library can be used by external crates (verify with simple test project)

---

### Iteration 2: Basic Rendering (Planned)
**Goal**: Render a triangle on screen with proper resource management
**Duration**: 1 week
**Status**: 📋 BACKLOG
**Dependencies**: Iteration 1 (VulkanContext library)

#### Must Have (MVP)
- [ ] **Buffer Management** (Day 1)
  - [ ] Create `src/buffer.rs` with simple `Buffer` struct wrapping vulkano buffer
  - [ ] Implement RAII cleanup with `Drop` trait
  - [ ] Add type-safe vertex buffer creation helper
  - [ ] Basic error handling for buffer allocation failures

- [ ] **Shader System** (Day 2)
  - [ ] Create `src/shader.rs` with embedded SPIR-V loading
  - [ ] Add simple vertex/fragment shader loading functions
  - [ ] Include basic triangle shaders as embedded bytes
  - [ ] Basic shader validation (compilation check only)

- [ ] **Pipeline Creation** (Day 3)
  - [ ] Create `src/pipeline.rs` with graphics pipeline wrapper
  - [ ] Implement simple pipeline creation for triangle rendering
  - [ ] Add render pass creation helper
  - [ ] Basic pipeline error handling

- [ ] **Rendering Integration** (Day 4)
  - [ ] Create `src/renderer.rs` with basic command recording
  - [ ] Implement simple draw command abstraction
  - [ ] Add frame synchronization helpers
  - [ ] Basic command submission

- [ ] **Triangle Example** (Day 5)
  - [ ] Create `examples/basic_triangle/` using gamma_vk library
  - [ ] Implement colored triangle with hardcoded vertices
  - [ ] Add window event handling for close/resize
  - [ ] Validate triangle renders correctly

#### Success Criteria (Measurable)
- [ ] Triangle renders correctly on local development machine
- [ ] `cargo test` passes for all buffer/shader/pipeline units
- [ ] Example runs without memory leaks (check with Activity Monitor)
- [ ] Library API is intuitive (<50 lines for triangle example)
- [ ] Code maintains zero clippy warnings

---

### Iteration 3: Error Handling Improvements (Planned)
**Goal**: Iteratively improve error handling and add basic texture support
**Duration**: 1 week
**Status**: 📋 BACKLOG
**Dependencies**: Iteration 2 (Basic Rendering)

#### Must Have (MVP)
- [ ] **Enhanced Error Handling** (Day 1-2)
  - [ ] Add error context and source chaining to `GammaVkError`
  - [ ] Implement error recovery patterns for common failures
  - [ ] Add debug formatting and error reporting utilities
  - [ ] Create error testing utilities and validation

- [ ] **Basic Texture System** (Day 3-4)
  - [ ] Create `src/texture.rs` with simple texture wrapper
  - [ ] Add basic image loading from embedded data or simple formats
  - [ ] Implement texture creation with RAII cleanup
  - [ ] Add basic sampler creation and management

- [ ] **Textured Quad Example** (Day 5)
  - [ ] Create `examples/textured_quad/` using gamma_vk library
  - [ ] Load and display a simple texture on a quad
  - [ ] Add UV coordinate handling in vertex data
  - [ ] Validate textured rendering works correctly

#### Success Criteria (Measurable)
- [ ] Error messages provide clear, actionable information
- [ ] Can load and display basic textures (PNG/JPEG)
- [ ] Textured quad renders correctly on local machine
- [ ] Memory usage remains stable during texture operations
- [ ] All examples build and run without warnings

---

### Future Iterations (Deferred)
**Items moved to later iterations when complexity is justified:**

#### Advanced Features (TBD)
- [ ] **Cross-Platform Testing**: When hardware/CI resources available
- [ ] **CI/CD Pipeline**: When team size or release cadence justifies automation
- [ ] **Advanced Memory Management**: Resource pooling, allocation strategies
- [ ] **Performance Optimization**: Benchmarking, profiling, optimization
- [ ] **Advanced Rendering**: Multiple primitives, scene graphs, advanced shaders

---

## Development Guidelines

### Daily Workflow
1. **Check TODO.md** for current iteration tasks
2. **Update task status** as work progresses
3. **Run tests** before committing changes
4. **Update documentation** for new features
5. **Review code quality** with clippy and fmt

### Task Status Legend
- 📋 **BACKLOG**: Not yet started, planned for future
- 🔄 **PLANNED**: Defined and ready to start
- 🚧 **IN PROGRESS**: Currently being worked on
- 🔍 **REVIEW**: Completed, awaiting review/testing
- ✅ **COMPLETED**: Finished and verified
- ❌ **BLOCKED**: Cannot proceed due to dependencies
- 🔄 **DEFERRED**: Moved to later iteration

### Quality Standards
- All public APIs must have rustdoc documentation
- Unit tests required for all non-trivial functionality
- Integration tests for end-to-end scenarios
- No clippy warnings in default configuration
- Code formatted with rustfmt
- Memory safety verified (no unsafe without justification)

### Daily Workflow (Local Development)
1. **Morning standup** (self): Review yesterday's progress, plan today's tasks
2. **Quality check**: Run `cargo test && cargo clippy && cargo fmt` before starting
3. **Work in small increments**: Commit working code frequently
4. **End-of-day validation**: Ensure all quality gates still pass
5. **Update TODO.md**: Mark completed tasks, note any blockers or changes

### Risk Management (Simplified)
- **Technical Risks**: Document unknowns and research solutions (spikes if needed)
- **Scope Creep**: Stick to iteration goals; defer nice-to-haves to future iterations
- **Quality Debt**: Address failing tests and clippy warnings immediately
- **Complexity Creep**: Keep solutions simple; optimize and abstract later
- **Vulkan Compatibility**: Design for cross-platform, test locally first

### Dependency Management
Simple linear progression:
- **Iteration 1 → 2**: Library must exist before building rendering on top
- **Iteration 2 → 3**: Basic rendering must work before adding textures/error improvements
- **Each day**: Previous day's work must be complete and tested

### Rollback Strategy
If any iteration becomes blocked:
1. **Assess scope**: Can it be reduced to meet timeline?
2. **Defer complexity**: Move advanced features to future iterations
3. **Maintain working state**: Always keep examples functional
4. **Document lessons**: Update TODO.md with findings for future reference

## Notes and Decisions

### Architecture Decisions
- **RAII Pattern**: All resources use automatic lifetime management
- **Type Safety**: Distinct types for different resource usages
- **Error Handling**: Comprehensive error types with recovery strategies
- **Modularity**: Functional organization, clear public interfaces

### Technical Notes
- MoltenVK requires `khr_portability_enumeration` extension
- Current setup successfully initializes Vulkan on macOS
- Using Vulkano 0.35.1 for Vulkan abstraction
- Winit 0.30 for window management

### Future Considerations (Post-Iteration 3)
- Async resource loading for large assets
- Multi-threaded command recording
- Plugin system for custom rendering techniques
- Integration with popular game engines
- Advanced rendering techniques (PBR, shadows, post-processing)
- Scene graph and spatial optimization

---

## Iteration Health Tracking

### Current Iteration Metrics
- **Velocity**: Track story points completed per iteration
- **Quality**: Monitor test coverage, clippy warnings, performance regressions
- **Predictability**: Measure estimation accuracy (planned vs actual duration)
- **Risk**: Count blockers encountered and resolution time

### Weekly Checkpoints
Every iteration should have mid-week health checks:
1. **Progress Assessment**: Are we on track for iteration goals?
2. **Blocker Review**: What's blocking progress and how to resolve?
3. **Quality Check**: Are quality gates being maintained?
4. **Scope Assessment**: Do we need to adjust Must Have vs Should Have?

---

## Contacts and Resources

### Documentation
- [Architecture Overview](docs/DESIGN_PRINCIPLES.md)
- [Style Guide](docs/STYLE_GUIDE.md)
- [Iterative Development Guide](docs/ITERATIVE_DEVELOPMENT.md)
- [RAII Pattern Guide](docs/RAII_PATTERN.md)
- [Project Structure](docs/PROJECT_STRUCTURE.md)

### External Resources
- [Vulkano Documentation](https://docs.rs/vulkano/)
- [Vulkan Specification](https://registry.khronos.org/vulkan/)
- [Learn Vulkan](https://vulkan-tutorial.com/)

### Staff Engineer Review Notes
- **First Review**: Original Iteration 1 was too ambitious (6 weeks planned for 2 weeks)
- **Second Review**: Removed unnecessary complexity (CI/CD, mocking, cross-platform)
- **Final Approach**: Focus on working software over perfect process
- **Key Deferrals**: CI/CD, cross-platform testing, advanced error handling, complex infrastructure
- **Pragmatic Priorities**: Local development, real Vulkan testing, iterative improvements

### Design Philosophy
- **Start Simple**: Basic library first, optimize later
- **Real Testing**: Use actual Vulkan instead of mocking
- **Local First**: Perfect local development before adding complexity
- **Iterative Error Handling**: Improve error handling incrementally, not upfront
- **Justify Complexity**: Only add infrastructure when benefits are clear

Last Updated: 2024-06-25 (Revised based on second staff engineer review)