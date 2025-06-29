# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Gamma-VK is a safe, performant Vulkan graphics engine built in Rust. The project follows structured iterative development with a pragmatic, test-driven approach.

**Key Characteristics**:
- **Test-Driven Development**: Tests written before implementation
- **RAII Resource Management**: Automatic GPU resource cleanup via Rust's ownership system  
- **Type-Safe APIs**: Distinct types prevent common graphics programming errors
- **Iterative Architecture**: Building incrementally from foundation to full engine

**For Current Status**: Always check [TODO.md](TODO.md) for iteration progress and active tasks

## Development Commands

### Referencing Documentation

always feel free to check appropriate documentation if you are unsure about how to implement a command or feature

### Searching Local Documentation

```bash
# Index all documentation (first time or after major changes)
python tools/semantic_search.py index .

# Index specific directories
python tools/semantic_search.py index docs/

# Index only specific file types
python tools/semantic_search.py index . --extensions .md .txt

# Search for concepts semantically
python tools/semantic_search.py search "how to handle buffer allocation"
python tools/semantic_search.py search "error handling patterns"
python tools/semantic_search.py search "vulkan initialization steps"

# Get more results
python tools/semantic_search.py search "memory management" -n 10

# Check index statistics
python tools/semantic_search.py stats
```

### Building and Running
```bash
# Build the project
cargo build

# Run the main application (Hello World demo)
cargo run

# Build in release mode
cargo build --release

# Check compilation without building
cargo check
```

### Testing Commands
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name::

# Run integration tests only
cargo test --test integration_test_name
```

### Development Tools
```bash
# Format code according to project style
cargo fmt

# Run clippy for additional linting
cargo clippy

# Check for security vulnerabilities
cargo audit  # (if cargo-audit is installed)

# Generate documentation
cargo doc --open
```

### Iteration-Specific Commands
```bash
# Check current iteration status and tasks
cat TODO.md | head -20

# View current iteration detailed tasks
grep -A 20 "Must Have (MVP)" TODO.md

# Check quality standards compliance
cargo clippy -- -D warnings  # No warnings allowed
cargo fmt --check             # Code must be formatted
cargo test                    # All tests must pass
```

## Architecture Overview

Gamma-VK follows a layered architecture designed around three core principles: **Safety by Default**, **Performance by Design**, and **Extensible by Nature**.

### High-Level Architecture Layers

1. **Platform Layer** - Window management, input handling, file system abstraction
2. **Vulkan Abstraction** - Device management, memory allocation, synchronization primitives
3. **Graphics Abstraction** - Pipeline management, resource management, command buffers
4. **Rendering Engine** - Scene management, material system, camera system
5. **Application Layer** - User-facing APIs and example applications

### Key Architectural Concepts

#### RAII Resource Management
All GPU resources use automatic lifetime management through Rust's ownership system. Resources are acquired during object construction and automatically released when objects are dropped. This prevents resource leaks and ensures proper cleanup even in error scenarios.

#### Type-Safe Resource Usage
The type system prevents common graphics programming errors by using distinct types for different buffer usages (e.g., `VertexBuffer`, `IndexBuffer`, `UniformBuffer`) rather than generic buffer types.

#### Zero-Cost Abstractions
High-level APIs compile down to optimal Vulkan calls without runtime overhead. Generic code and trait implementations are monomorphized at compile time.

#### Extension Points
The architecture supports extensibility through:
- **Trait-based plugins** for custom rendering techniques
- **Custom allocators** for specialized memory management
- **Asset loaders** for different file formats
- **Debug markers** for profiling and debugging

### Current Module Structure

The project currently has a flat module structure in `src/`:

- `context.rs` - Vulkan instance, device, and surface management
- `buffer.rs` - Type-safe buffer abstractions (VertexBuffer, IndexBuffer, UniformBuffer)
- `shader.rs` - Shader module loading from SPIR-V files
- `error.rs` - Comprehensive error type hierarchy
- `lib.rs` - Public API exports

### Future Architecture Vision

The project will evolve toward a hierarchical module structure:

- `core/` - Expanded context and device management
- `memory/` - Advanced allocation strategies and pooling
- `command/` - Command buffer recording and synchronization
- `pipeline/` - Pipeline caching and state management
- `render/` - High-level rendering abstractions
- Additional modules as the engine grows

## MoltenVK Configuration

The project includes special handling for MoltenVK (Vulkan on macOS):

```rust
// Required for MoltenVK support
enabled_extensions: InstanceExtensions {
    khr_portability_enumeration: true,
    ..InstanceExtensions::empty()
},
flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
```

This configuration is essential for Vulkan initialization on macOS systems.

## Error Handling Strategy

The project uses a comprehensive error type hierarchy with `GammaVkError` that provides:
- Vulkan-specific error wrapping
- Resource management errors (out of memory, resource not found)
- Pipeline errors (shader compilation, pipeline creation)
- Platform errors (window creation, surface incompatibility)
- Configuration errors with actionable feedback

Error recovery follows patterns of graceful degradation, automatic cleanup with RAII, and context recovery for device loss scenarios.

## Development Methodology

### IMPORTANT
- Don't remove "unused" parameters without understanding architectural purpose
- Always check debug files when working on related modules

### Debug Notes Usage

When working on specific modules or encountering issues, consult relevant debug files for lessons learned and architectural insights:

#### Auto-Context Rules
- **Buffer work**: Read `debug/debug_buffer.md` + `debug/debug_vulkano_api.md`
- **Type errors**: Check `debug/debug_rust_types.md` 
- **Architecture decisions**: Reference `debug/debug_architecture.md`
- **Vulkano API confusion**: Start with `debug/debug_vulkano_api.md`

#### Trigger Patterns
- Seeing "unused parameter" → Load `debug_architecture.md`
- Type mismatch errors (`u64` vs `usize`) → Load `debug_rust_types.md`
- Vulkano buffer creation issues → Load `debug_vulkano_api.md` + `debug_buffer.md`
- Questioning architectural decisions → Load `debug_architecture.md`

#### When to Update Debug Files
- API gotchas discovered through documentation
- Architectural decisions that might seem unclear later
- Type system issues and their resolutions
- Wrong assumptions corrected during development

### Test-Driven Development (TDD)

The project has adopted TDD as its primary development methodology. This means:

1. **Write Tests First**: Define expected behavior through tests before implementing
2. **Red-Green-Refactor Cycle**:
   - Red: Write a failing test
   - Green: Write minimal code to make it pass
   - Refactor: Improve code while keeping tests green
3. **Comprehensive Test Coverage**: Both unit tests and integration tests
4. **Test as Documentation**: Tests serve as executable specifications

#### TDD Workflow Example
```rust
// 1. Write the test first (defines the API)
#[test]
fn test_vertex_buffer_creation() {
    let buffer = VertexBuffer::<Vertex>::new(&context, &vertices)?;
    assert_eq!(buffer.len(), vertices.len());
}

// 2. Then implement to make it pass
```

### Iterative Development Approach
The project follows a structured iterative development methodology with:
- **Time-boxed iterations** focused on delivering working, tested functionality
- **Test-driven development** with comprehensive unit and integration testing
- **Quality gates** that must be met before task completion
- **Incremental architecture** building from foundation to advanced features
- **Pragmatic approach**: Working software over perfect process

### Development Phases

The project progresses through planned iterations that build upon each other:

1. **Foundation** - Core architecture, error handling, Vulkan context management
2. **Basic Rendering** - Triangle rendering with shader system and pipelines  
3. **Resource Management** - Texture system and advanced memory management
4. **Advanced Features** - Scene management, materials, and optimization

**Always check [TODO.md](TODO.md) for current iteration and specific tasks**

### Daily Session Workflow
1. Check [TODO.md](TODO.md) for current iteration and tasks
2. Create/update session log when starting significant work
3. Update task status as work progresses  
4. Run quality checks before committing (`cargo test`, `cargo clippy`, `cargo fmt`)
5. Update documentation for new features
6. Follow iterative development principles from [docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md)
7. **Session End**: Document insights and decisions in session_logs/{i}_{task_name}.log.md

### Session Logging
Session insights and architectural decisions are documented in:
- **session_logs/{i}_{task_name}.log.md** - Numbered session logs capturing key decisions, analysis, and next steps

### Quality Standards
- All public APIs must have rustdoc documentation
- Unit tests required for all non-trivial functionality  
- Integration tests for end-to-end scenarios
- No clippy warnings in default configuration
- Code formatted with rustfmt
- Memory safety verified (no unsafe without justification)

## Code Style and Conventions

### Naming Conventions
- **Types/Traits**: PascalCase (`VulkanContext`, `ResourceManager`)
- **Functions/Variables**: snake_case (`create_render_pass`, `buffer_size`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_FRAMES_IN_FLIGHT`)
- **Modules**: snake_case (`vulkan_context`, `resource_manager`)

### Import Organization
```rust
// Standard library imports first
use std::collections::HashMap;
use std::sync::Arc;

// External crate imports
use vulkano::device::Device;
use winit::window::Window;

// Local crate imports
use crate::context::VulkanContext;
use crate::error::GammaVkError;
```

### Struct Organization
```rust
pub struct Example {
    // Public fields first
    pub name: String,
    
    // Private fields last
    internal_state: State,
}

impl Example {
    // Associated functions first
    pub fn new() -> Self { }
    
    // Public methods
    pub fn update(&mut self) { }
    
    // Private methods last
    fn internal_helper(&self) { }
}
```

## Documentation Standards

- Use comprehensive rustdoc comments with examples
- Include safety documentation for unsafe code blocks
- Document error conditions and recovery strategies
- Provide usage examples for complex APIs
- Explain performance characteristics and trade-offs

## Extension Development

When adding new functionality:

1. **Follow the trait-based extension pattern** for pluggable components
2. **Implement proper RAII** for any new resource types
3. **Use builder patterns** for complex configuration objects
4. **Provide multiple abstraction levels** (high-level convenience, mid-level control, low-level escape hatches)
5. **Consider cross-platform compatibility** and feature detection

The project architecture is designed to support incremental development and extension without requiring major architectural changes.

## Key Documentation

- **[TODO.md](TODO.md)** - Current iteration status, tasks, and development roadmap
- **[session_logs/](session_logs/)** - Development session logs with key insights and decisions
- **[debug/](debug/)** - Debug notes capturing API gotchas, architectural decisions, and lessons learned
- **[docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md)** - Comprehensive iterative development methodology
- **[docs/DESIGN_PRINCIPLES.md](docs/DESIGN_PRINCIPLES.md)** - Core architectural principles and design philosophy
- **[docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)** - Detailed coding standards and conventions
- **[docs/RAII_PATTERN.md](docs/RAII_PATTERN.md)** - Deep dive into RAII resource management patterns
- **[docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)** - File structure guideline - establishes consistent organization patterns