# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Prompting Execution of Workflows

if a prompt starts with `WF `, it means to follow the prompts defined in the matching workflow found in [/workflows](workflows/*)

### examples
- `WF session start` = `follow the prompts defined in /workflows/session_start.workflow`
- `WF simple_critical_analysis` = `follow the prompts defined in /workflows/simple_critical_analysis.workflow`
- `WF quality gate` = `follow the prompts defined in /workflows/quality_gate.workflow`
- `WF task_planning` = `follow the prompts defined in /workflows/task_planning.workflow`

- `WF quick review` = (custom alias for simple_critical_analysis) `follow the prompts defined in /workflows/simple_critical_analysis.workflow`


## Project Overview

Gamma-VK is a safe, performant Vulkan graphics engine built in Rust. The project follows structured iterative development with 2-week iterations, currently transitioning from a working "Hello World" Vulkan application to a proper library architecture with comprehensive testing and documentation.

**Current Status**: See [TODO.md](TODO.md) for detailed iteration progress, current tasks, and development roadmap.

## Development Commands

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

### Module Organization

The codebase is organized by functionality rather than type:

- `core/` - Vulkan context, device, and surface management
- `memory/` - Memory allocation, buffers, images, resource pooling
- `command/` - Command buffer management and synchronization
- `pipeline/` - Graphics/compute pipelines, shaders, caching
- `render/` - High-level rendering, scene management, cameras
- `geometry/` - Mesh handling, vertex formats, primitives
- `texture/` - Texture management, samplers, atlases
- `platform/` - Platform-specific implementations
- `util/` - Math, timing, configuration utilities

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

### Iterative Development Approach
The project follows a structured iterative development methodology with:
- **2-week iterations** focused on delivering working, tested functionality
- **Test-driven development** with comprehensive unit and integration testing
- **Quality gates** that must be met before task completion
- **Incremental architecture** building from foundation to advanced features

### Current Development Phase
**See [TODO.md](TODO.md) for up-to-date iteration status and task details.**

The project is organized into planned iterations:
1. **Foundation** - Core architecture, error handling, Vulkan context management
2. **Basic Rendering** - Triangle rendering with shader system and pipelines  
3. **Resource Management** - Texture system and advanced memory management

### Daily Workflow
1. **Session Start**: Run `workflows/session_start.workflow` for comprehensive session initialization
2. Check [TODO.md](TODO.md) for current iteration tasks (part of session start workflow)
3. Update task status as work progresses  
4. Run quality checks before committing (`cargo test`, `cargo clippy`, `cargo fmt`)
5. Update documentation for new features
6. Follow iterative development principles from [docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md)
7. **Session End**: Document insights and decisions in workflow_sessions/{i}_{task_name}.log

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

## AI Workflow Management System

The project includes a structured workflow system to ensure consistent, high-quality development sessions with staff engineer-level analysis.

### Workflow Files
- **[workflows/session_start.workflow](workflows/session_start.workflow)** - Comprehensive session initialization procedure
- **[workflows/iteration_review.workflow](workflows/iteration_review.workflow)** - Mid-iteration health checks and risk assessment
- **[workflows/task_planning.workflow](workflows/task_planning.workflow)** - Complex task breakdown and planning methodology
- **[workflows/quality_gate.workflow](workflows/quality_gate.workflow)** - Pre-commit quality validation checklist

### Session Logging
Session insights and architectural decisions are documented in:
- **workflow_sessions/{i}_{task_name}.log** - Numbered session logs capturing key decisions, analysis, and next steps

### Usage Examples
```bash
# Start a development session (recommended)
# Follow procedures in workflows/session_start.workflow

# Mid-iteration health check
# Follow procedures in workflows/iteration_review.workflow

# Planning complex tasks
# Follow procedures in workflows/task_planning.workflow

# Pre-commit validation
# Follow procedures in workflows/quality_gate.workflow
```

### Benefits
- **Consistency**: Standardized procedures across all development sessions
- **Knowledge Retention**: Session logs preserve architectural decisions and insights
- **Quality Assurance**: Systematic checks prevent oversights and regressions
- **Staff Engineer Perspective**: Built-in critical analysis and long-term thinking

## Key Documentation

- **[TODO.md](TODO.md)** - Current iteration status, tasks, and development roadmap
- **[workflow_sessions/](workflow_sessions/)** - Development session logs with key insights and decisions
- **[docs/ITERATIVE_DEVELOPMENT.md](docs/ITERATIVE_DEVELOPMENT.md)** - Comprehensive iterative development methodology
- **[docs/DESIGN_PRINCIPLES.md](docs/DESIGN_PRINCIPLES.md)** - Core architectural principles and design philosophy
- **[docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)** - Detailed coding standards and conventions
- **[docs/RAII_PATTERN.md](docs/RAII_PATTERN.md)** - Deep dive into RAII resource management patterns
- **[docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md)** - File structure guideline - establishes consistent organization patterns