# Gamma-VK Project Structure

## Overview

This document defines the canonical project structure for Gamma-VK, establishing consistent organization patterns that support scalability, maintainability, and clear separation of concerns.

## Directory Structure

```
gamma-vk/
├── Cargo.toml                 # Main project manifest
├── README.md                  # Project overview and quick start
├── LICENSE                    # Project license
├── CHANGELOG.md               # Version history and changes
├── 
├── papers/                    # Interesting stuff to read - unrelated to project documentation
│   └── cool_research.md
│
├── docs/                      # Documentation
│   ├── STYLE_GUIDE.md         # Coding standards and conventions
│   ├── DESIGN_PRINCIPLES.md   # Core architectural principles
│   ├── PROJECT_STRUCTURE.md   # This file
│   ├── API_REFERENCE.md       # API documentation
│   ├── CONTRIBUTING.md        # Contributor guidelines
│   └── examples/              # Documentation examples
│       ├── basic_triangle.md
│       ├── texture_loading.md
│       └── custom_shaders.md
│
├── src/                       # Source code
│   ├── lib.rs                 # Main library entry point
│   ├── 
│   ├── core/                  # Core engine systems
│   │   ├── mod.rs
│   │   ├── context.rs         # Vulkan context management
│   │   ├── device.rs          # Device selection and management
│   │   ├── surface.rs         # Surface creation and management
│   │   └── validation.rs      # Debug validation systems
│   │
│   ├── memory/                # Memory management
│   │   ├── mod.rs
│   │   ├── allocator.rs       # Memory allocation strategies
│   │   ├── buffer.rs          # Buffer management
│   │   ├── image.rs           # Image/texture management
│   │   └── pool.rs            # Resource pooling
│   │
│   ├── command/               # Command buffer management
│   │   ├── mod.rs
│   │   ├── encoder.rs         # High-level command encoding
│   │   ├── buffer.rs          # Command buffer abstraction
│   │   ├── pool.rs            # Command pool management
│   │   └── sync.rs            # Synchronization primitives
│   │
│   ├── pipeline/              # Pipeline management
│   │   ├── mod.rs
│   │   ├── graphics.rs        # Graphics pipeline
│   │   ├── compute.rs         # Compute pipeline
│   │   ├── cache.rs           # Pipeline caching
│   │   ├── layout.rs          # Pipeline layouts
│   │   └── builder.rs         # Pipeline builders
│   │
│   ├── shader/                # Shader management
│   │   ├── mod.rs
│   │   ├── module.rs          # Shader module wrapper
│   │   ├── compiler.rs        # Shader compilation
│   │   ├── reflection.rs      # Shader reflection
│   │   └── cache.rs           # Compiled shader cache
│   │
│   ├── render/                # High-level rendering
│   │   ├── mod.rs
│   │   ├── pass.rs            # Render pass abstraction
│   │   ├── frame.rs           # Frame management
│   │   ├── scene.rs           # Scene management
│   │   ├── camera.rs          # Camera systems
│   │   ├── light.rs           # Lighting systems
│   │   └── material.rs        # Material system
│   │
│   ├── geometry/              # Geometry and mesh handling
│   │   ├── mod.rs
│   │   ├── mesh.rs            # Mesh representation
│   │   ├── vertex.rs          # Vertex formats
│   │   ├── primitive.rs       # Basic geometric primitives
│   │   └── loader.rs          # Mesh loading utilities
│   │
│   ├── texture/               # Texture management
│   │   ├── mod.rs
│   │   ├── texture.rs         # Texture abstraction
│   │   ├── sampler.rs         # Sampler management
│   │   ├── loader.rs          # Image loading
│   │   └── atlas.rs           # Texture atlas management
│   │
│   ├── util/                  # Utilities and helpers
│   │   ├── mod.rs
│   │   ├── math.rs            # Mathematical utilities
│   │   ├── hash.rs            # Hashing utilities
│   │   ├── time.rs            # Timing and profiling
│   │   └── config.rs          # Configuration management
│   │
│   ├── platform/              # Platform-specific code
│   │   ├── mod.rs
│   │   ├── windows.rs         # Windows-specific implementations
│   │   ├── macos.rs           # macOS-specific implementations
│   │   ├── linux.rs           # Linux-specific implementations
│   │   └── common.rs          # Common platform utilities
│   │
│   ├── debug/                 # Debug and profiling tools
│   │   ├── mod.rs
│   │   ├── marker.rs          # Debug markers
│   │   ├── profiler.rs        # GPU profiling
│   │   ├── validator.rs       # Runtime validation
│   │   └── inspector.rs       # Resource inspection
│   │
│   └── error.rs               # Error types and handling
│
├── examples/                  # Example applications
│   ├── basic_triangle/        # Basic triangle rendering
│   │   ├── main.rs
│   │   └── Cargo.toml
│   ├── texture_quad/          # Textured quad example
│   │   ├── main.rs
│   │   ├── assets/
│   │   └── Cargo.toml
│   ├── compute_shader/        # Compute shader example
│   │   ├── main.rs
│   │   └── Cargo.toml
│   └── pbr_rendering/         # PBR rendering example
│       ├── main.rs
│       ├── shaders/
│       ├── assets/
│       └── Cargo.toml
│
├── tests/                     # Integration tests
│   ├── context_creation.rs    # Context creation tests
│   ├── memory_management.rs   # Memory management tests
│   ├── pipeline_creation.rs   # Pipeline creation tests
│   ├── rendering.rs           # End-to-end rendering tests
│   └── common/                # Test utilities
│       ├── mod.rs
│       ├── mock_device.rs
│       └── test_helpers.rs
│
├── benches/                   # Benchmarks
│   ├── memory_allocation.rs   # Memory allocation benchmarks
│   ├── pipeline_creation.rs   # Pipeline creation benchmarks
│   ├── command_recording.rs   # Command recording benchmarks
│   └── rendering.rs           # Rendering performance benchmarks
│
├── assets/                    # Shared assets for examples/tests
│   ├── shaders/               # SPIR-V shader files
│   │   ├── vertex/
│   │   ├── fragment/
│   │   └── compute/
│   ├── textures/              # Test textures
│   └── models/                # Test 3D models
│
├── tools/                     # Development tools
│   ├── shader_compiler/       # Shader compilation tool
│   ├── asset_processor/       # Asset processing pipeline
│   └── performance_analyzer/  # Performance analysis tools
│
└── .github/                   # GitHub specific files
    ├── workflows/             # CI/CD workflows
    │   ├── ci.yml
    │   ├── release.yml
    │   └── docs.yml
    ├── ISSUE_TEMPLATE/        # Issue templates
    └── PULL_REQUEST_TEMPLATE.md
```

## Module Organization Principles

### 1. Functional Grouping
Modules are organized by functionality rather than by type:

```rust
// Good: Functional grouping
mod memory {
    pub mod allocator;
    pub mod buffer;
    pub mod image;
}

// Avoid: Type-based grouping
mod structs {
    // All structs together
}
mod traits {
    // All traits together
}
```

### 2. Layered Dependencies
Lower layers should not depend on higher layers:

```
Application Layer (examples, user code)
    ↓
High-Level API (render, scene, camera)
    ↓
Mid-Level API (pipeline, command, shader)
    ↓
Low-Level API (memory, core, platform)
    ↓
External Dependencies (vulkano, winit)
```

### 3. Clear Public Interfaces
Each module should have a clear, documented public interface:

```rust
// src/memory/mod.rs
pub use self::allocator::{Allocator, DefaultAllocator};
pub use self::buffer::{Buffer, BufferBuilder};
pub use self::image::{Image, ImageBuilder};

// Re-export commonly used types
pub use self::pool::ResourcePool;

// Private module - implementation detail
mod pool;
```

## File Naming Conventions

### Source Files
- **`mod.rs`**: Module root, contains public interface and re-exports
- **`lib.rs`**: Crate root, defines the public API
- **`[feature].rs`**: Implementation of a specific feature (e.g., `allocator.rs`)
- **`builder.rs`**: Builder pattern implementations
- **`error.rs`**: Error types for the module

### Test Files
- **`tests/[module]_test.rs`**: Integration tests for a module
- **`tests/common/mod.rs`**: Shared test utilities
- **`benches/[feature]_bench.rs`**: Performance benchmarks

### Example Files
- **`examples/[name]/main.rs`**: Example application entry point
- **`examples/[name]/Cargo.toml`**: Example-specific dependencies

## Dependency Management

### Cargo.toml Structure
```toml
[package]
name = "gamma-vk"
version = "0.1.0"
edition = "2021"
authors = ["Gamma-VK Contributors"]
description = "A safe, performant Vulkan graphics engine for Rust"
license = "MIT OR Apache-2.0"
repository = "https://github.com/gamma-vk/gamma-vk"
documentation = "https://docs.rs/gamma-vk"
keywords = ["vulkan", "graphics", "rendering", "3d", "gpu"]
categories = ["graphics", "game-development", "multimedia"]

[dependencies]
# Core Vulkan bindings
vulkano = "0.35"

# Math library
glam = "0.25"

# Error handling
thiserror = "1.0"

# Serialization (optional)
serde = { version = "1.0", features = ["derive"], optional = true }

# Async runtime (optional)
tokio = { version = "1.0", features = ["rt-multi-thread"], optional = true }

# Platform support
winit = "0.30"
raw-window-handle = "0.6"

[dev-dependencies]
# Testing utilities
proptest = "1.0"
criterion = "0.5"

[features]
default = ["serde"]
debug = ["vulkano/debug"]
validation = []
async = ["tokio"]
serde = ["dep:serde"]

[[example]]
name = "basic_triangle"
path = "examples/basic_triangle/main.rs"

[[bench]]
name = "memory_allocation"
harness = false
```

### Feature Flags
- **`default`**: Standard features for most users
- **`debug`**: Additional debugging capabilities
- **`validation`**: Runtime validation (for development)
- **`async`**: Async/await support for resource loading
- **`serde`**: Serialization support for assets

## Configuration Management

### Build-time Configuration
```rust
// src/config.rs
pub struct BuildConfig {
    pub enable_validation: bool,
    pub enable_debug_markers: bool,
    pub max_frames_in_flight: u32,
}

impl BuildConfig {
    #[cfg(debug_assertions)]
    pub const fn debug() -> Self {
        Self {
            enable_validation: true,
            enable_debug_markers: true,
            max_frames_in_flight: 2,
        }
    }
    
    #[cfg(not(debug_assertions))]
    pub const fn release() -> Self {
        Self {
            enable_validation: false,
            enable_debug_markers: false,
            max_frames_in_flight: 3,
        }
    }
}
```

### Runtime Configuration
```rust
// Environment variable support
pub fn load_config() -> EngineConfig {
    EngineConfig {
        max_frames_in_flight: env::var("GAMMA_VK_MAX_FRAMES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2),
        
        enable_validation: env::var("GAMMA_VK_VALIDATION")
            .map(|s| s == "1" || s.to_lowercase() == "true")
            .unwrap_or(false),
    }
}
```

## Documentation Structure

### README.md Template
```markdown
# Gamma-VK

A safe, performant Vulkan graphics engine for Rust.

## Features
- Memory-safe Vulkan abstractions
- Zero-cost performance
- Extensible architecture

## Quick Start
```rust
use gamma_vk::*;

let context = VulkanContext::new()?;
let renderer = Renderer::new(context)?;
// ...
```

## Examples
- [Basic Triangle](examples/basic_triangle/)
- [Texture Loading](examples/texture_quad/)

## Documentation
- [API Reference](https://docs.rs/gamma-vk)
- [Architecture Guide](docs/ARCHITECTURE.md)
```

This project structure provides a solid foundation that can scale from a simple graphics library to a full-featured rendering engine while maintaining clarity and organization.