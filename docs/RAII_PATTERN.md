# RAII Pattern in Gamma-VK

## Overview

RAII (Resource Acquisition Is Initialization) is a fundamental programming pattern that ties resource management to object lifetime. In the context of graphics programming and Vulkan, RAII provides automatic, safe, and deterministic cleanup of GPU resources, preventing memory leaks and ensuring proper resource management even in complex error scenarios.

## What is RAII?

RAII is a programming idiom where:
1. **Resources are acquired during object construction**
2. **Resources are automatically released during object destruction**
3. **Object lifetime determines resource lifetime**

In Rust, RAII is implemented through the `Drop` trait, which provides automatic cleanup when values go out of scope.

## Basic RAII Implementation

### Simple Resource Management
```rust
pub struct Buffer {
    vulkan_buffer: VkBuffer,
    memory: VkDeviceMemory,
    device: Arc<Device>,
    size: u64,
    debug_name: String,
}

impl Buffer {
    pub fn new(device: Arc<Device>, size: u64, usage: BufferUsage) -> Result<Self, GammaVkError> {
        // Resource acquisition happens during construction
        let buffer_info = BufferCreateInfo {
            size,
            usage: usage.into(),
            ..Default::default()
        };
        
        let vulkan_buffer = unsafe {
            device.create_buffer(&buffer_info, None)
                .map_err(GammaVkError::VulkanError)?
        };
        
        let memory_requirements = unsafe {
            device.get_buffer_memory_requirements(vulkan_buffer)
        };
        
        let memory = allocate_memory(&device, memory_requirements)?;
        
        unsafe {
            device.bind_buffer_memory(vulkan_buffer, memory, 0)
                .map_err(GammaVkError::VulkanError)?;
        }
        
        Ok(Buffer {
            vulkan_buffer,
            memory,
            device,
            size,
            debug_name: format!("Buffer_{:?}", vulkan_buffer),
        })
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // Resource deallocation happens automatically during destruction
        println!("Destroying {}", self.debug_name);
        
        unsafe {
            self.device.destroy_buffer(self.vulkan_buffer, None);
            self.device.free_memory(self.memory, None);
        }
    }
}

// Usage - completely automatic cleanup!
fn create_and_use_buffer() -> Result<(), GammaVkError> {
    let buffer = Buffer::new(device, 1024, BufferUsage::VERTEX)?;
    
    // Use the buffer for rendering...
    upload_vertex_data(&buffer, &vertex_data)?;
    
    // Buffer is automatically cleaned up here when it goes out of scope
    // No manual cleanup code required!
    Ok(())
} // <- Buffer::drop() called automatically here
```

## RAII Benefits in Graphics Programming

### 1. Automatic Resource Cleanup

The most obvious benefit is that resources are automatically cleaned up:

```rust
fn render_frame(scene: &Scene) -> Result<(), GammaVkError> {
    // Multiple resources acquired
    let command_buffer = CommandBuffer::new(&command_pool)?;
    let render_target = RenderTarget::new(&device, 1920, 1080)?;
    let depth_buffer = DepthBuffer::new(&device, 1920, 1080)?;
    
    // Complex rendering operations...
    command_buffer.begin_render_pass(&render_target)?;
    command_buffer.set_depth_buffer(&depth_buffer)?;
    
    for object in scene.objects() {
        let material = Material::load(object.material_path())?;
        let mesh = Mesh::load(object.mesh_path())?;
        
        command_buffer.bind_material(&material)?;
        command_buffer.draw_mesh(&mesh)?;
        
        // material and mesh are automatically cleaned up at end of loop iteration
    }
    
    command_buffer.end_render_pass()?;
    
    // ALL resources automatically cleaned up when function exits:
    // - command_buffer
    // - render_target  
    // - depth_buffer
    // No manual cleanup code needed!
    Ok(())
}
```

### 2. Exception Safety

RAII ensures cleanup even when errors occur:

```rust
fn complex_rendering_operation() -> Result<(), GammaVkError> {
    let pipeline = GraphicsPipeline::create(&device, &pipeline_desc)?;
    let descriptor_set = DescriptorSet::allocate(&descriptor_pool)?;
    let uniform_buffer = Buffer::new(device.clone(), 256, BufferUsage::UNIFORM)?;
    
    // Upload uniform data
    uniform_buffer.map_memory()?;
    uniform_buffer.write_data(&uniform_data)?;
    uniform_buffer.unmap_memory()?;
    
    // This might fail, but resources are still cleaned up!
    risky_gpu_operation(&pipeline, &descriptor_set, &uniform_buffer)?;
    
    Ok(())
    
    // Even if risky_gpu_operation() fails:
    // - pipeline is automatically destroyed
    // - descriptor_set is automatically freed
    // - uniform_buffer is automatically destroyed
    // No resource leaks!
}
```

### 3. Scope-Based Resource Management

Resources can be tied to specific scopes for precise lifetime control:

```rust
fn multi_pass_rendering(scene: &Scene) -> Result<(), GammaVkError> {
    // Shadow mapping pass
    {
        let shadow_map = ShadowMap::new(&device, 2048, 2048)?;
        let shadow_pipeline = ShadowPipeline::create(&device)?;
        
        render_shadows(scene, &shadow_map, &shadow_pipeline)?;
        
        // Store shadow map for later use
        scene.set_shadow_map(shadow_map.texture().clone());
        
    } // shadow_pipeline automatically destroyed here, shadow_map GPU memory freed
    
    // Geometry pass
    {
        let gbuffer = GBuffer::new(&device, screen_width, screen_height)?;
        let geometry_pipeline = GeometryPipeline::create(&device)?;
        
        render_geometry(scene, &gbuffer, &geometry_pipeline)?;
        
        // Store gbuffer for lighting pass
        scene.set_gbuffer(gbuffer.textures().clone());
        
    } // geometry_pipeline and gbuffer GPU memory automatically cleaned up
    
    // Lighting pass
    {
        let lighting_pipeline = LightingPipeline::create(&device)?;
        let final_target = RenderTarget::new(&device, screen_width, screen_height)?;
        
        render_lighting(scene, &final_target, &lighting_pipeline)?;
        
        present_frame(&final_target)?;
        
    } // lighting_pipeline and final_target automatically cleaned up
    
    Ok(())
}
```

### 4. Hierarchical Resource Management

RAII supports complex resource hierarchies:

```rust
pub struct RenderPass {
    render_pass: VkRenderPass,
    framebuffers: Vec<Framebuffer>,
    device: Arc<Device>,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        // Framebuffers are dropped first (because they're owned)
        // Then render_pass is destroyed
        unsafe {
            self.device.destroy_render_pass(self.render_pass, None);
        }
    }
}

pub struct Framebuffer {
    framebuffer: VkFramebuffer,
    attachments: Vec<ImageView>,
    device: Arc<Device>,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        // ImageViews are dropped first
        // Then framebuffer is destroyed
        unsafe {
            self.device.destroy_framebuffer(self.framebuffer, None);
        }
    }
}

pub struct ImageView {
    image_view: VkImageView,
    image: Arc<Image>,
    device: Arc<Device>,
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_image_view(self.image_view, None);
        }
        // Image is reference-counted and destroyed when last reference is dropped
    }
}
```

## Comparison with Manual Management

### Without RAII (C-style Vulkan)
```c
VkResult render_frame() {
    VkBuffer vertex_buffer;
    VkDeviceMemory vertex_memory;
    VkBuffer index_buffer;
    VkDeviceMemory index_memory;
    VkDescriptorSet descriptor_set;
    VkCommandBuffer command_buffer;
    
    // Manual resource acquisition
    VkResult result = vkCreateBuffer(device, &vertex_info, NULL, &vertex_buffer);
    if (result != VK_SUCCESS) return result;
    
    result = vkAllocateMemory(device, &vertex_alloc_info, NULL, &vertex_memory);
    if (result != VK_SUCCESS) {
        vkDestroyBuffer(device, vertex_buffer, NULL); // Must remember cleanup!
        return result;
    }
    
    result = vkCreateBuffer(device, &index_info, NULL, &index_buffer);
    if (result != VK_SUCCESS) {
        vkFreeMemory(device, vertex_memory, NULL);    // Must remember cleanup!
        vkDestroyBuffer(device, vertex_buffer, NULL); // Must remember cleanup!
        return result;
    }
    
    // ... more error-prone resource creation ...
    
    // Complex rendering logic here...
    
    // Manual cleanup - easy to forget or get wrong!
    vkFreeDescriptorSets(device, descriptor_pool, 1, &descriptor_set);
    vkFreeCommandBuffers(device, command_pool, 1, &command_buffer);
    vkDestroyBuffer(device, index_buffer, NULL);
    vkFreeMemory(device, index_memory, NULL);
    vkDestroyBuffer(device, vertex_buffer, NULL);
    vkFreeMemory(device, vertex_memory, NULL);
    
    return VK_SUCCESS;
}
```

### With RAII (Rust-style)
```rust
fn render_frame() -> Result<(), GammaVkError> {
    let vertex_buffer = Buffer::new(device.clone(), vertex_data.len(), BufferUsage::VERTEX)?;
    let index_buffer = Buffer::new(device.clone(), index_data.len(), BufferUsage::INDEX)?;
    let descriptor_set = DescriptorSet::allocate(&descriptor_pool)?;
    let command_buffer = CommandBuffer::new(&command_pool)?;
    
    // Upload data
    vertex_buffer.upload(&vertex_data)?;
    index_buffer.upload(&index_data)?;
    
    // Complex rendering logic here...
    command_buffer.bind_vertex_buffer(&vertex_buffer)?;
    command_buffer.bind_index_buffer(&index_buffer)?;
    command_buffer.bind_descriptor_set(&descriptor_set)?;
    command_buffer.draw_indexed(index_data.len())?;
    
    Ok(())
    
    // ALL cleanup happens automatically in correct order:
    // 1. command_buffer is freed
    // 2. descriptor_set is freed  
    // 3. index_buffer is destroyed
    // 4. vertex_buffer is destroyed
    // No manual cleanup code required!
}
```

## Advanced RAII Patterns

### Smart Pointers for Shared Resources

```rust
// Shared ownership with automatic cleanup
pub struct Texture {
    image: VkImage,
    image_view: VkImageView,
    memory: VkDeviceMemory,
    device: Arc<Device>,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_image_view(self.image_view, None);
            self.device.destroy_image(self.image, None);
            self.device.free_memory(self.memory, None);
        }
    }
}

// Multiple objects can share the same texture
pub struct Material {
    diffuse_texture: Arc<Texture>,
    normal_texture: Arc<Texture>,
    roughness_texture: Arc<Texture>,
}

// Usage
fn create_materials() -> Result<Vec<Material>, GammaVkError> {
    let shared_normal = Arc::new(Texture::load("default_normal.png")?);
    
    let materials = vec![
        Material {
            diffuse_texture: Arc::new(Texture::load("wood_diffuse.png")?),
            normal_texture: shared_normal.clone(), // Shared reference
            roughness_texture: Arc::new(Texture::load("wood_roughness.png")?),
        },
        Material {
            diffuse_texture: Arc::new(Texture::load("metal_diffuse.png")?),
            normal_texture: shared_normal.clone(), // Shared reference
            roughness_texture: Arc::new(Texture::load("metal_roughness.png")?),
        },
    ];
    
    Ok(materials)
    
    // shared_normal is only destroyed when ALL materials are dropped
}
```

### Custom Drop Guards

```rust
pub struct DebugMarker {
    name: String,
    command_buffer: *mut VkCommandBuffer,
}

impl DebugMarker {
    pub fn new(command_buffer: &mut CommandBuffer, name: &str) -> Self {
        unsafe {
            // Begin debug marker
            let debug_info = VkDebugMarkerBeginInfoEXT {
                marker_name: name.as_ptr() as *const i8,
                color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            };
            vkCmdDebugMarkerBeginEXT(command_buffer.handle(), &debug_info);
        }
        
        DebugMarker {
            name: name.to_string(),
            command_buffer: command_buffer.handle_mut(),
        }
    }
}

impl Drop for DebugMarker {
    fn drop(&mut self) {
        unsafe {
            // Automatically end debug marker when scope ends
            vkCmdDebugMarkerEndEXT(self.command_buffer);
        }
        println!("Debug marker '{}' ended", self.name);
    }
}

// Usage
fn render_with_debug_markers(command_buffer: &mut CommandBuffer) -> Result<(), GammaVkError> {
    {
        let _geometry_marker = DebugMarker::new(command_buffer, "Geometry Pass");
        render_geometry(command_buffer)?;
    } // Geometry marker automatically ended here
    
    {
        let _lighting_marker = DebugMarker::new(command_buffer, "Lighting Pass");
        render_lighting(command_buffer)?;
    } // Lighting marker automatically ended here
    
    Ok(())
}
```

### Resource Pools with RAII

```rust
pub struct PooledResource<T> {
    resource: Option<T>,
    pool: Arc<Mutex<ResourcePool<T>>>,
}

impl<T> PooledResource<T> {
    pub fn get(&self) -> &T {
        self.resource.as_ref().unwrap()
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        self.resource.as_mut().unwrap()
    }
}

impl<T: Reusable> Drop for PooledResource<T> {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            // Reset resource state for reuse
            resource.reset();
            
            // Return to pool
            self.pool.lock().unwrap().return_resource(resource);
        }
    }
}

pub struct ResourcePool<T> {
    available: Vec<T>,
    create_fn: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T: Reusable> ResourcePool<T> {
    pub fn acquire(&mut self) -> PooledResource<T> {
        let resource = self.available.pop()
            .unwrap_or_else(|| (self.create_fn)());
        
        PooledResource {
            resource: Some(resource),
            pool: self.pool.clone(),
        }
    }
}

// Usage
fn use_pooled_command_buffer() -> Result<(), GammaVkError> {
    let mut command_buffer = command_buffer_pool.acquire();
    
    command_buffer.begin()?;
    command_buffer.draw_triangle()?;
    command_buffer.end()?;
    
    submit_commands(&command_buffer)?;
    
    // Command buffer automatically returned to pool when dropped
    Ok(())
}
```

## Why RAII is Essential for Vulkan

Vulkan requires careful management of many different resource types:

### Core Resources
- **VkInstance** - The Vulkan instance
- **VkDevice** - Logical device
- **VkQueue** - Command queues

### Memory Resources  
- **VkBuffer** - Vertex buffers, index buffers, uniform buffers
- **VkImage** - Textures, render targets, depth buffers
- **VkDeviceMemory** - GPU memory allocations

### Pipeline Resources
- **VkShaderModule** - Compiled shaders
- **VkPipelineLayout** - Pipeline resource layouts
- **VkRenderPass** - Render pass descriptions
- **VkPipeline** - Graphics and compute pipelines

### Command Resources
- **VkCommandPool** - Command buffer pools
- **VkCommandBuffer** - Command buffers
- **VkDescriptorPool** - Descriptor pools
- **VkDescriptorSet** - Descriptor sets

### Synchronization Resources
- **VkSemaphore** - GPU-GPU synchronization
- **VkFence** - GPU-CPU synchronization
- **VkEvent** - Fine-grained synchronization

Without RAII, managing all these resources manually is:
- **Error-prone** - Easy to forget cleanup or get order wrong
- **Verbose** - Lots of repetitive cleanup code
- **Unsafe** - Resource leaks and use-after-free bugs
- **Complex** - Difficult to handle error paths correctly

With RAII, resource management becomes:
- **Automatic** - Resources cleaned up without manual intervention
- **Safe** - No resource leaks or use-after-free possible
- **Simple** - Clean, readable code focused on logic, not cleanup
- **Robust** - Correct cleanup even in error scenarios

## Best Practices for RAII in Gamma-VK

### 1. Always Implement Drop for Resource-Owning Types
```rust
// Good: Drop implementation for cleanup
pub struct Pipeline {
    pipeline: VkPipeline,
    device: Arc<Device>,
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);
        }
    }
}
```

### 2. Use Arc for Shared Resources
```rust
// Good: Shared device reference
pub struct Buffer {
    device: Arc<Device>, // Reference-counted shared ownership
    buffer: VkBuffer,
}

// Avoid: Raw references with lifetime parameters
pub struct Buffer<'a> {
    device: &'a Device, // Complex lifetime management
    buffer: VkBuffer,
}
```

### 3. Prefer Owned Data Over Borrowing
```rust
// Good: Owned debug name
pub struct Resource {
    debug_name: String, // Owned, no lifetime issues
}

// Avoid: Borrowed data with lifetimes
pub struct Resource<'a> {
    debug_name: &'a str, // Lifetime complexity
}
```

### 4. Use Scope Guards for Temporary Operations
```rust
pub struct RenderPassScope<'a> {
    command_buffer: &'a mut CommandBuffer,
}

impl<'a> RenderPassScope<'a> {
    pub fn new(command_buffer: &'a mut CommandBuffer, render_pass: &RenderPass) -> Self {
        command_buffer.begin_render_pass(render_pass);
        RenderPassScope { command_buffer }
    }
}

impl Drop for RenderPassScope<'_> {
    fn drop(&mut self) {
        self.command_buffer.end_render_pass();
    }
}

// Usage ensures render pass is always properly ended
fn render_scene(command_buffer: &mut CommandBuffer, scene: &Scene) -> Result<(), GammaVkError> {
    let _render_pass = RenderPassScope::new(command_buffer, &scene.render_pass());
    
    // Render operations...
    command_buffer.draw_mesh(&scene.mesh())?;
    
    Ok(())
} // Render pass automatically ended here
```

## Vulkano-Specific RAII Patterns

### Subbuffer RAII Pattern

Vulkano provides `Subbuffer<[T]>` which automatically handles RAII for buffer resources:

```rust
use vulkano::{
    buffer::{Buffer as VulkanoBuffer, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

pub struct Buffer {
    // Subbuffer automatically handles cleanup
    buffer: Subbuffer<[u8]>,
}

impl Buffer {
    pub fn new_host_visible(
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
        usage: BufferUsage,
    ) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator,
            BufferCreateInfo { usage, ..Default::default() },
            AllocationCreateInfo {
                // Critical: Specify memory type for host-visible access
                memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            size,
        )?;

        Ok(Buffer { buffer })
    }
}

// No explicit Drop implementation needed!
// Subbuffer<[u8]> handles all cleanup automatically
```

### Memory Type Stratification

Vulkano requires explicit memory allocation strategies. RAII works differently for different memory types:

```rust
impl Buffer {
    // Host-visible: CPU can write directly
    pub fn new_host_visible(/* ... */) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator,
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            size,
        )?;
        Ok(Buffer { buffer })
    }

    // Device-local: GPU optimal, requires staging for CPU writes
    pub fn new_device_local(/* ... */) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator,
            create_info,
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            size,
        )?;
        Ok(Buffer { buffer })
    }
}
```

### Type-Safe Buffer Wrappers

RAII combined with type safety prevents misuse:

```rust
pub struct VertexBuffer {
    buffer: Buffer,  // Wraps RAII buffer
}

impl VertexBuffer {
    pub fn new_host_visible(
        device: Arc<Device>,
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        // Usage flags enforced at construction
        let buffer = Buffer::new_host_visible(device, allocator, size, BufferUsage::VERTEX_BUFFER)?;
        Ok(VertexBuffer { buffer })
    }
}

// Usage:
fn render_mesh() -> Result<()> {
    let vertex_buffer = VertexBuffer::new_host_visible(device, allocator, 1024)?;
    let vertex_data = vec![1.0f32, 2.0, 3.0];
    
    // Type system prevents using vertex buffer as index buffer
    vertex_buffer.buffer().write_data(&vertex_data.as_bytes())?;
    
    // Buffer automatically cleaned up when it goes out of scope
    Ok(())
} // <- VertexBuffer dropped here, Subbuffer cleanup triggered
```

### Vulkano Error Handling with RAII

Vulkano errors integrate seamlessly with RAII patterns:

```rust
fn create_rendering_resources() -> Result<(), GammaVkError> {
    let vertex_buffer = Buffer::new_host_visible(device.clone(), allocator.clone(), 1024, BufferUsage::VERTEX_BUFFER)?;
    let index_buffer = Buffer::new_host_visible(device.clone(), allocator.clone(), 512, BufferUsage::INDEX_BUFFER)?;
    let uniform_buffer = Buffer::new_host_visible(device, allocator, 256, BufferUsage::UNIFORM_BUFFER)?;
    
    // Upload data to buffers
    vertex_buffer.write_data(&vertex_data)?;
    index_buffer.write_data(&index_data)?;
    uniform_buffer.write_data(&uniform_data)?;
    
    // If any operation fails, ALL buffers are automatically cleaned up
    // No manual cleanup needed even in error scenarios
    
    Ok(())
}
```

### Memory Allocation RAII

Vulkano's allocator pattern ensures proper memory management:

```rust
// Allocator is shared and automatically manages memory lifecycle
let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

{
    // Multiple buffers can share the same allocator
    let buffer1 = Buffer::new_host_visible(device.clone(), allocator.clone(), 1024, BufferUsage::VERTEX_BUFFER)?;
    let buffer2 = Buffer::new_device_local(device.clone(), allocator.clone(), 2048, BufferUsage::INDEX_BUFFER | BufferUsage::TRANSFER_DST)?;
    
    // Buffers automatically cleaned up here
}

// Allocator remains valid for future use
let buffer3 = Buffer::new_host_visible(device, allocator, 512, BufferUsage::UNIFORM_BUFFER)?;
```

### Key Vulkano RAII Insights

1. **`Subbuffer<[T]>` provides automatic cleanup** - No manual Drop implementation needed
2. **Memory type filters are critical** - `AllocationCreateInfo::default()` often fails
3. **Type wrappers enhance safety** - Prevent buffer misuse at compile time
4. **Error handling works seamlessly** - Failed allocations don't leak resources
5. **Allocator sharing is safe** - Reference counting handles lifecycle automatically

These patterns ensure that Vulkano resources are managed safely and efficiently, following Rust's ownership model while leveraging Vulkano's automatic resource management capabilities.

RAII is fundamental to safe, efficient graphics programming in Rust. By tying resource lifetimes to object lifetimes, we eliminate entire classes of bugs while making code cleaner and more maintainable. In the context of Vulkan, where resource management is complex and critical, RAII transforms error-prone manual cleanup into automatic, guaranteed correctness.