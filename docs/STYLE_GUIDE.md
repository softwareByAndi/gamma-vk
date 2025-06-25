# Gamma-VK Style Guide

## Overview

This document establishes coding standards, architectural principles, and best practices for the Gamma-VK project. These guidelines ensure consistency, maintainability, and safety across the codebase.

## Rust Style Guidelines

### Naming Conventions

#### Types and Traits
```rust
// Use PascalCase for types, structs, enums, and traits
pub struct VulkanContext { }
pub enum RenderError { }
pub trait ResourceManager { }

// Use descriptive names that indicate purpose
pub struct FrameData { }           // Good
pub struct Data { }                // Avoid - too generic
```

#### Functions and Variables
```rust
// Use snake_case for functions, methods, and variables
fn create_render_pass() -> Result<RenderPass> { }
let buffer_size = calculate_buffer_size();

// Use descriptive names
fn allocate_vertex_buffer() { }    // Good
fn alloc_buf() { }                 // Avoid - unclear abbreviations
```

#### Constants and Statics
```rust
// Use SCREAMING_SNAKE_CASE for constants
const MAX_FRAMES_IN_FLIGHT: u32 = 2;
const DEFAULT_QUEUE_FAMILY_INDEX: u32 = 0;

// Group related constants
pub mod limits {
    pub const MAX_DESCRIPTOR_SETS: u32 = 1000;
    pub const MAX_VERTEX_ATTRIBUTES: u32 = 16;
}
```

#### Modules
```rust
// Use snake_case for module names
mod vulkan_context;
mod resource_manager;
mod command_buffer;

// Organize modules by functionality, not by type
mod rendering {          // Good - functional grouping
    mod pipeline;
    mod material;
    mod mesh;
}

mod structs {           // Avoid - grouping by type
    // ...
}
```

### Code Organization

#### Module Structure
```rust
// Order imports: std, external crates, local crates, self
use std::collections::HashMap;
use std::sync::Arc;

use vulkano::device::Device;
use winit::window::Window;

use crate::context::VulkanContext;
use crate::error::GammaVkError;

// Group and order declarations
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

#### Error Handling
```rust
// Use Result types for fallible operations
pub fn create_buffer(size: u64) -> Result<Buffer, GammaVkError> {
    // Implementation
}

// Use ? operator for early returns
pub fn complex_operation() -> Result<(), GammaVkError> {
    let buffer = create_buffer(1024)?;
    let texture = load_texture("path")?;
    // ...
    Ok(())
}

// Provide context for errors
pub fn load_shader(path: &Path) -> Result<ShaderModule, GammaVkError> {
    std::fs::read(path)
        .map_err(|e| GammaVkError::FileNotFound { 
            path: path.to_path_buf(), 
            source: e 
        })?;
    // ...
}
```

### Safety and Ownership

#### Smart Pointers
```rust
// Use Arc for shared ownership across threads
pub struct SharedResource {
    device: Arc<Device>,
    buffer: Arc<Buffer>,
}

// Use Rc for single-threaded shared ownership
use std::rc::Rc;
pub struct LocalResource {
    data: Rc<RefCell<Data>>,
}

// Prefer owned data when possible
pub struct Config {
    name: String,           // Good - owned
    description: String,
}

pub struct Config {
    name: &'static str,     // OK for static data
    description: &str,      // Avoid - lifetime complexity
}
```

#### Lifetime Management
```rust
// Minimize explicit lifetimes
pub struct Renderer<'a> {      // Avoid if possible
    context: &'a VulkanContext,
}

pub struct Renderer {          // Prefer owned/Arc
    context: Arc<VulkanContext>,
}

// Use clear lifetime names when needed
pub struct View<'frame> {
    frame_data: &'frame FrameData,
}
```

#### Unsafe Code
```rust
// Isolate unsafe code in small, well-documented functions
/// # Safety
/// The caller must ensure that `ptr` is valid and properly aligned
unsafe fn raw_buffer_access(ptr: *mut u8, size: usize) -> &mut [u8] {
    std::slice::from_raw_parts_mut(ptr, size)
}

// Provide safe wrappers
pub struct MappedBuffer {
    ptr: *mut u8,
    size: usize,
}

impl MappedBuffer {
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        // Safety: Guaranteed by construction
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}
```

## Architecture Principles

### Composition over Inheritance
```rust
// Prefer composition with traits
pub trait Renderable {
    fn render(&self, encoder: &mut CommandEncoder) -> Result<()>;
}

pub struct Mesh {
    vertices: Buffer,
    indices: Buffer,
}

impl Renderable for Mesh {
    fn render(&self, encoder: &mut CommandEncoder) -> Result<()> {
        // Implementation
    }
}

// Avoid deep inheritance hierarchies
// Instead, compose smaller traits
pub trait Drawable: Renderable + Cullable + Sortable { }
```

### Dependency Injection
```rust
// Prefer generics for zero-cost abstraction
pub struct Renderer<A: MemoryAllocator, L: AssetLoader> {
    allocator: A,
    loader: L,
}

// only use trait objects for flexibility where generics are not appropriate
pub struct Renderer {
    allocator: Box<dyn MemoryAllocator>,
    loader: Box<dyn AssetLoader>,
}

impl Renderer {
    pub fn new(
        allocator: Box<dyn MemoryAllocator>,
        loader: Box<dyn AssetLoader>,
    ) -> Self {
        Self { allocator, loader }
    }
}

```

### Builder Pattern
```rust
// Use builder pattern for complex configuration
pub struct PipelineBuilder {
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
    vertex_layout: Option<VertexLayout>,
    render_pass: Option<RenderPass>,
    // ... other fields
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn vertex_shader(mut self, shader: ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }
    
    pub fn fragment_shader(mut self, shader: ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }
    
    pub fn build(self) -> Result<Pipeline, GammaVkError> {
        let vertex_shader = self.vertex_shader
            .ok_or(GammaVkError::MissingVertexShader)?;
        // ... validation and construction
    }
}

// Usage
let pipeline = PipelineBuilder::new()
    .vertex_shader(vs)
    .fragment_shader(fs)
    .vertex_layout(layout)
    .build()?;
```

### Error Handling Strategy
```rust
// Define a comprehensive error type
#[derive(Debug, thiserror::Error)]
pub enum GammaVkError {
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vulkano::VulkanError),
    
    #[error("Out of memory: device={device}, host={host}")]
    OutOfMemory { device: bool, host: bool },
    
    #[error("Resource not found: {id}")]
    ResourceNotFound { id: String },
    
    #[error("Shader compilation failed: {stage:?} - {message}")]
    ShaderCompilation { stage: ShaderStage, message: String },
    
    #[error("Configuration error in field '{field}': {message}")]
    Configuration { field: String, message: String },
}

// Provide conversion methods for common error types
impl From<std::io::Error> for GammaVkError {
    fn from(err: std::io::Error) -> Self {
        GammaVkError::Io(err)
    }
}
```

## Testing Guidelines

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_creation() {
        let allocator = MockAllocator::new();
        let buffer = Buffer::new(&allocator, 1024, BufferUsage::VERTEX);
        
        assert!(buffer.is_ok());
        assert_eq!(buffer.unwrap().size(), 1024);
    }
    
    #[test]
    fn test_buffer_creation_failure() {
        let allocator = MockAllocator::with_failure();
        let result = Buffer::new(&allocator, 1024, BufferUsage::VERTEX);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GammaVkError::OutOfMemory { .. }));
    }
}
```

### Integration Tests
```rust
// Place integration tests in tests/ directory
// tests/rendering.rs
use gamma_vk::*;

#[test]
fn test_basic_rendering_pipeline() {
    let context = VulkanContext::new().unwrap();
    let renderer = Renderer::new(context);
    
    let mesh = create_test_triangle();
    let result = renderer.render_frame(&[mesh]);
    
    assert!(result.is_ok());
}
```

## Documentation Standards

### Rustdoc Comments
```rust
/// Creates a new vertex buffer with the specified size and usage.
/// 
/// # Arguments
/// 
/// * `allocator` - The memory allocator to use for buffer creation
/// * `size` - Size of the buffer in bytes
/// * `usage` - Intended usage of the buffer
/// 
/// # Returns
/// 
/// Returns a `Result` containing the created buffer or an error if allocation fails.
/// 
/// # Examples
/// 
/// ```
/// use gamma_vk::*;
/// 
/// let allocator = DefaultAllocator::new();
/// let buffer = Buffer::new(&allocator, 1024, BufferUsage::VERTEX)?;
/// ```
/// 
/// # Errors
/// 
/// This function will return an error if:
/// * The allocator runs out of memory
/// * The requested size exceeds device limits
/// * The usage flags are incompatible
pub fn create_buffer(
    allocator: &dyn MemoryAllocator,
    size: u64,
    usage: BufferUsage,
) -> Result<Buffer, GammaVkError> {
    // Implementation
}
```

### Code Comments
```rust
// Use comments to explain WHY, not WHAT
pub fn optimize_mesh(mesh: &mut Mesh) {
    // Sort vertices by spatial locality to improve cache performance
    mesh.vertices.sort_by(|a, b| a.position.x.partial_cmp(&b.position.x));
    
    // Generate vertex cache optimization
    // This reduces the number of vertex shader invocations
    mesh.optimize_vertex_cache();
}
```

## Performance Guidelines

### Memory Management
```rust
// Prefer stack allocation when possible
fn process_vertices(vertices: &[Vertex]) {
    let mut transformed = Vec::with_capacity(vertices.len()); // Pre-allocate
    // Process vertices...
}

// Use object pools for frequently allocated resources
pub struct CommandBufferPool {
    available: Vec<CommandBuffer>,
    in_use: Vec<CommandBuffer>,
}

impl CommandBufferPool {
    pub fn acquire(&mut self) -> CommandBuffer {
        self.available.pop()
            .unwrap_or_else(|| self.create_new_buffer())
    }
    
    pub fn release(&mut self, buffer: CommandBuffer) {
        buffer.reset();
        self.available.push(buffer);
    }
}
```

### Async and Concurrency
```rust
// Use tokio for async operations
pub async fn load_texture_async(path: &Path) -> Result<Texture, GammaVkError> {
    let data = tokio::fs::read(path).await?;
    let image = decode_image(&data)?;
    Ok(Texture::from_image(image))
}

// Use parking_lot for better mutex performance
use parking_lot::RwLock;

pub struct ResourceCache {
    textures: RwLock<HashMap<String, Arc<Texture>>>,
}

impl ResourceCache {
    pub fn get_texture(&self, name: &str) -> Option<Arc<Texture>> {
        self.textures.read().get(name).cloned()
    }
}
```

This style guide provides the foundation for consistent, maintainable, and performant code throughout the Gamma-VK project.