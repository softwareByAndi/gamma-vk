//! Buffer management for Gamma-VK
//!
//! This module provides RAII-managed buffer types with automatic resource cleanup
//! and type-safe buffer usage patterns.

use std::sync::Arc;
use vulkano::{
    buffer::{Buffer as VulkanoBuffer, BufferCreateInfo, BufferUsage, Subbuffer},
    device::Device,
    memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator},
};

use crate::{GammaVkError, Result};

/// A managed buffer wrapper providing RAII resource management
///
/// Buffer wraps a Vulkano buffer and provides automatic cleanup through
/// Rust's ownership system. It ensures proper resource lifecycle management
/// and prevents memory leaks.
pub struct Buffer {
    /// The underlying Vulkano subbuffer
    buffer: Subbuffer<[u8]>,
}

impl Buffer {
    /// Create a new buffer with the specified size and usage
    ///
    /// # Arguments
    ///
    /// * `allocator` - Memory allocator for buffer allocation
    /// * `size` - Size of the buffer in bytes
    /// * `usage` - Intended usage flags for the buffer
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created buffer or an error if allocation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The allocator runs out of memory
    /// * The requested size exceeds device limits
    /// * The usage flags are invalid or unsupported
    pub fn new(
        _device: Arc<Device>,
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
        usage: BufferUsage,
    ) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator,
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
            size,
        )
        .map_err(|e| GammaVkError::buffer_creation(format!("Failed to create buffer: {}", e)))?;

        Ok(Buffer { buffer })
    }

    /// Get the size of the buffer in bytes
    pub fn size(&self) -> u64 {
        self.buffer.len()
    }

    /// Get the underlying Vulkano subbuffer
    ///
    /// This provides access to the raw buffer for advanced use cases
    /// while maintaining the RAII wrapper for automatic cleanup.
    pub fn inner(&self) -> &Subbuffer<[u8]> {
        &self.buffer
    }

    /// Write data to the buffer
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The data is larger than the buffer
    /// * Memory mapping fails
    pub fn write_data(&self, data: &[u8]) -> Result<()> {
        if data.len() > self.buffer.len() as usize {
            return Err(GammaVkError::buffer_creation(format!(
                "Data size {} exceeds buffer size {}",
                data.len(),
                self.buffer.len()
            )));
        }

        let mut write_lock = self.buffer.write().map_err(|e| {
            GammaVkError::buffer_creation(format!("Failed to lock buffer for writing: {}", e))
        })?;

        write_lock[..data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Get buffer usage flags
    pub fn usage(&self) -> BufferUsage {
        self.buffer.buffer().usage()
    }
}

impl Drop for Buffer {
    /// Automatic cleanup when Buffer is dropped
    ///
    /// This implementation ensures proper resource cleanup through Rust's RAII.
    /// The underlying Vulkano buffer will be automatically cleaned up when
    /// this buffer goes out of scope.
    fn drop(&mut self) {
        // Buffer resources are automatically cleaned up by Subbuffer
        // when it goes out of scope
    }
}

/// Type-safe vertex buffer wrapper
///
/// VertexBuffer prevents accidentally using vertex buffers in inappropriate contexts
/// and provides vertex-specific functionality.
pub struct VertexBuffer {
    buffer: Buffer,
}

impl VertexBuffer {
    /// Create a new vertex buffer
    ///
    /// # Arguments
    ///
    /// * `allocator` - Memory allocator for buffer allocation
    /// * `size` - Size of the buffer in bytes
    pub fn new(
        device: Arc<Device>,
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new(device, allocator, size, BufferUsage::VERTEX_BUFFER)?;
        Ok(VertexBuffer { buffer })
    }

    /// Get the underlying buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get the size of the vertex buffer
    pub fn size(&self) -> u64 {
        self.buffer.size()
    }
}

/// Type-safe index buffer wrapper
///
/// IndexBuffer prevents accidentally using index buffers in inappropriate contexts
/// and provides index-specific functionality.
pub struct IndexBuffer {
    buffer: Buffer,
}

impl IndexBuffer {
    /// Create a new index buffer
    ///
    /// # Arguments
    ///
    /// * `allocator` - Memory allocator for buffer allocation
    /// * `size` - Size of the buffer in bytes
    pub fn new(
        device: Arc<Device>,
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new(device, allocator, size, BufferUsage::INDEX_BUFFER)?;
        Ok(IndexBuffer { buffer })
    }

    /// Get the underlying buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get the size of the index buffer
    pub fn size(&self) -> u64 {
        self.buffer.size()
    }
}

/// Type-safe uniform buffer wrapper
///
/// UniformBuffer prevents accidentally using uniform buffers in inappropriate contexts
/// and provides uniform-specific functionality.
pub struct UniformBuffer {
    buffer: Buffer,
}

impl UniformBuffer {
    /// Create a new uniform buffer
    ///
    /// # Arguments
    ///
    /// * `allocator` - Memory allocator for buffer allocation
    /// * `size` - Size of the buffer in bytes
    pub fn new(
        device: Arc<Device>,
        allocator: Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new(device, allocator, size, BufferUsage::UNIFORM_BUFFER)?;
        Ok(UniformBuffer { buffer })
    }

    /// Get the underlying buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get the size of the uniform buffer
    pub fn size(&self) -> u64 {
        self.buffer.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vulkano::{
        device::{Device, DeviceCreateInfo, QueueCreateInfo},
        memory::allocator::StandardMemoryAllocator,
    };

    fn create_test_device() -> Option<(Arc<Device>, Arc<StandardMemoryAllocator>)> {
        // This test helper creates a device for testing
        // It may fail in CI environments without Vulkan drivers
        let library = vulkano::VulkanLibrary::new().ok()?;
        let instance = vulkano::instance::Instance::new(
            library,
            vulkano::instance::InstanceCreateInfo::default(),
        )
        .ok()?;

        let physical_device = instance.enumerate_physical_devices().ok()?.next()?;

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, q)| {
                q.queue_flags
                    .intersects(vulkano::device::QueueFlags::GRAPHICS)
            })?;

        let (device, _queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: queue_family_index as u32,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .ok()?;

        let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        Some((device, allocator))
    }

    #[test]
    fn test_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer = Buffer::new(device, allocator.clone(), 1024, BufferUsage::VERTEX_BUFFER);

            assert!(buffer.is_ok());
            let buffer = buffer.unwrap();
            assert_eq!(buffer.size(), 1024);
            assert!(buffer.usage().intersects(BufferUsage::VERTEX_BUFFER));
        } else {
            println!("Skipping buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_vertex_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let vertex_buffer = VertexBuffer::new(device, allocator.clone(), 2048);

            assert!(vertex_buffer.is_ok());
            let vertex_buffer = vertex_buffer.unwrap();
            assert_eq!(vertex_buffer.size(), 2048);
        } else {
            println!("Skipping vertex buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_index_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let index_buffer = IndexBuffer::new(device, allocator.clone(), 512);

            assert!(index_buffer.is_ok());
            let index_buffer = index_buffer.unwrap();
            assert_eq!(index_buffer.size(), 512);
        } else {
            println!("Skipping index buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_uniform_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let uniform_buffer = UniformBuffer::new(device, allocator.clone(), 256);

            assert!(uniform_buffer.is_ok());
            let uniform_buffer = uniform_buffer.unwrap();
            assert_eq!(uniform_buffer.size(), 256);
        } else {
            println!("Skipping uniform buffer creation test - no Vulkan device available");
        }
    }
}
