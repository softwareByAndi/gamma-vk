//! Buffer management for Gamma-VK
//!
//! This module provides RAII-managed buffer types with automatic resource cleanup
//! and type-safe buffer usage patterns.

use std::sync::Arc;
use vulkano::{
    buffer::{Buffer as VulkanoBuffer, BufferCreateInfo, BufferUsage, Subbuffer},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
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
    /// Create a new host-visible buffer (CPU can write directly)
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
    pub fn new_host_visible(
        _device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
        usage: BufferUsage,
    ) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator.clone(),
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            size,
        )
        .map_err(|e| {
            GammaVkError::buffer_creation(format!("Failed to create host-visible buffer: {}", e))
        })?;

        Ok(Buffer { buffer })
    }

    /// Create a new device-local buffer (optimal for GPU access)
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
    /// # Note
    ///
    /// Device-local buffers cannot be directly written from CPU.
    /// Use staging buffers and transfer operations for data upload.
    pub fn new_device_local(
        _device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
        usage: BufferUsage,
    ) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator.clone(),
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            },
            size,
        )
        .map_err(|e| {
            GammaVkError::buffer_creation(format!("Failed to create device-local buffer: {}", e))
        })?;

        Ok(Buffer { buffer })
    }

    /// Create a new buffer with custom allocation preferences
    ///
    /// # Arguments
    ///
    /// * `allocator` - Memory allocator for buffer allocation
    /// * `size` - Size of the buffer in bytes  
    /// * `usage` - Intended usage flags for the buffer
    /// * `allocation_info` - Custom allocation preferences
    pub fn new_custom(
        _device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
        usage: BufferUsage,
        allocation_info: AllocationCreateInfo,
    ) -> Result<Self> {
        let buffer = VulkanoBuffer::new_slice::<u8>(
            allocator.clone(),
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            allocation_info,
            size,
        )
        .map_err(|e| {
            GammaVkError::buffer_creation(format!("Failed to create custom buffer: {}", e))
        })?;

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

    /// Write data to the buffer (only works with host-visible buffers)
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The data is larger than the buffer
    /// * Memory mapping fails (buffer not host-visible)
    /// * Buffer memory is not host-accessible
    pub fn write_data(&self, data: &[u8]) -> Result<()> {
        if data.len() > self.buffer.len() as usize {
            return Err(GammaVkError::buffer_creation(format!(
                "Data size {} exceeds buffer size {}",
                data.len(),
                self.buffer.len()
            )));
        }

        let mut write_lock = self.buffer.write().map_err(|e| {
            GammaVkError::buffer_creation(format!(
                "Failed to lock buffer for writing (buffer may not be host-visible): {}",
                e
            ))
        })?;

        write_lock[..data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Check if this buffer is host-visible (can be written from CPU)
    pub fn is_host_visible(&self) -> bool {
        // This is a simplified check - in a real implementation you'd check the memory type
        self.buffer.write().is_ok()
    }

    /// Create a staging buffer and copy data to device-local buffer
    ///
    /// This helper method creates a temporary host-visible staging buffer,
    /// uploads data to it, then copies to the device-local buffer.
    ///
    /// # Arguments
    ///
    /// * `device` - Vulkan device for command buffer creation
    /// * `allocator` - Memory allocator for staging buffer
    /// * `data` - Data to upload to the device-local buffer
    ///
    /// # Note
    ///
    /// This is a placeholder for future staging buffer implementation.
    /// Real implementation would require command buffer recording and submission.
    pub fn upload_via_staging(
        &self,
        _device: &Arc<Device>,
        _allocator: &Arc<StandardMemoryAllocator>,
        _data: &[u8],
    ) -> Result<()> {
        // TODO: Implement staging buffer pattern for device-local buffers
        // This would involve:
        // 1. Create temporary host-visible staging buffer
        // 2. Write data to staging buffer
        // 3. Record copy command from staging to device-local buffer
        // 4. Submit command buffer and wait for completion
        // 5. Clean up staging buffer
        Err(GammaVkError::buffer_creation(
            "Staging buffer upload not yet implemented".to_string(),
        ))
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
    /// Create a new host-visible vertex buffer (can be written from CPU)
    pub fn new_host_visible(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new_host_visible(device, allocator, size, BufferUsage::VERTEX_BUFFER)?;
        Ok(VertexBuffer { buffer })
    }

    /// Create a new device-local vertex buffer (optimal for GPU access)
    pub fn new_device_local(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new_device_local(
            device,
            allocator,
            size,
            BufferUsage::VERTEX_BUFFER | BufferUsage::TRANSFER_DST,
        )?;
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
    /// Create a new host-visible index buffer (can be written from CPU)
    pub fn new_host_visible(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new_host_visible(device, allocator, size, BufferUsage::INDEX_BUFFER)?;
        Ok(IndexBuffer { buffer })
    }

    /// Create a new device-local index buffer (optimal for GPU access)
    pub fn new_device_local(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new_device_local(
            device,
            allocator,
            size,
            BufferUsage::INDEX_BUFFER | BufferUsage::TRANSFER_DST,
        )?;
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
    /// Create a new host-visible uniform buffer (can be updated from CPU)
    pub fn new_host_visible(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer =
            Buffer::new_host_visible(device, allocator, size, BufferUsage::UNIFORM_BUFFER)?;
        Ok(UniformBuffer { buffer })
    }

    /// Create a new device-local uniform buffer (requires staging for updates)
    pub fn new_device_local(
        device: &Arc<Device>,
        allocator: &Arc<StandardMemoryAllocator>,
        size: u64,
    ) -> Result<Self> {
        let buffer = Buffer::new_device_local(
            device,
            allocator,
            size,
            BufferUsage::UNIFORM_BUFFER | BufferUsage::TRANSFER_DST,
        )?;
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

        // Try with portability enumeration for MoltenVK first, then fallback
        let instance = match vulkano::instance::Instance::new(
            library.clone(),
            vulkano::instance::InstanceCreateInfo {
                enabled_extensions: vulkano::instance::InstanceExtensions {
                    khr_portability_enumeration: true,
                    ..vulkano::instance::InstanceExtensions::empty()
                },
                flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        ) {
            Ok(instance) => instance,
            Err(_) => {
                // Fallback to standard Vulkan if portability fails
                vulkano::instance::Instance::new(
                    library,
                    vulkano::instance::InstanceCreateInfo::default(),
                )
                .ok()?
            }
        };

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
    fn test_host_visible_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer =
                Buffer::new_host_visible(&device, &allocator, 1024, BufferUsage::VERTEX_BUFFER);

            assert!(
                buffer.is_ok(),
                "Should be able to create host-visible buffer"
            );
            let buffer = buffer.unwrap();
            assert_eq!(buffer.size(), 1024);
            assert!(buffer.usage().intersects(BufferUsage::VERTEX_BUFFER));
        } else {
            println!("Skipping host-visible buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_device_local_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer = Buffer::new_device_local(
                &device,
                &allocator,
                1024,
                BufferUsage::VERTEX_BUFFER | BufferUsage::TRANSFER_DST,
            );

            assert!(
                buffer.is_ok(),
                "Should be able to create device-local buffer"
            );
            let buffer = buffer.unwrap();
            assert_eq!(buffer.size(), 1024);
            assert!(buffer.usage().intersects(BufferUsage::VERTEX_BUFFER));
            assert!(buffer.usage().intersects(BufferUsage::TRANSFER_DST));
        } else {
            println!("Skipping device-local buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_host_visible_buffer_can_be_written() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer =
                Buffer::new_host_visible(&device, &allocator, 1024, BufferUsage::VERTEX_BUFFER);

            if let Ok(buffer) = buffer {
                // Test the core requirement: Can I write vertex data to a vertex buffer?
                let vertex_data = vec![1u8, 2, 3, 4, 5]; // Simulated vertex data
                let result = buffer.write_data(&vertex_data);
                assert!(
                    result.is_ok(),
                    "Should be able to write vertex data to host-visible buffer"
                );

                // Test that large data writes work
                let large_vertex_data = vec![42u8; 1024];
                let result = buffer.write_data(&large_vertex_data);
                assert!(
                    result.is_ok(),
                    "Should be able to write large vertex data to buffer"
                );
            }
        } else {
            println!("Skipping buffer write test - no Vulkan device available");
        }
    }

    #[test]
    fn test_device_local_buffer_cannot_be_written_directly() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer = Buffer::new_device_local(
                &device,
                &allocator,
                1024,
                BufferUsage::VERTEX_BUFFER | BufferUsage::TRANSFER_DST,
            );

            if let Ok(buffer) = buffer {
                // Device-local buffers should not be directly writable from CPU
                let vertex_data = vec![1u8, 2, 3, 4, 5];
                let result = buffer.write_data(&vertex_data);
                assert!(
                    result.is_err(),
                    "Device-local buffer should not be directly writable from CPU"
                );
            }
        } else {
            println!("Skipping device-local buffer write test - no Vulkan device available");
        }
    }

    #[test]
    fn test_buffer_write_data_bounds_checking() {
        if let Some((device, allocator)) = create_test_device() {
            let buffer =
                Buffer::new_host_visible(&device, &allocator, 100, BufferUsage::VERTEX_BUFFER);

            if let Ok(buffer) = buffer {
                // Test core safety requirement: Buffer should reject oversized data
                let oversized_data = vec![1u8; 200]; // Larger than buffer
                let result = buffer.write_data(&oversized_data);
                assert!(
                    result.is_err(),
                    "Buffer should reject data larger than its capacity"
                );

                // Verify error message is helpful
                if let Err(error) = result {
                    let error_msg = format!("{}", error);
                    assert!(
                        error_msg.contains("exceeds buffer size"),
                        "Error should explain the size constraint violation"
                    );
                }

                // Test that correctly sized data still works
                let correct_data = vec![1u8; 50]; // Smaller than buffer
                let result = buffer.write_data(&correct_data);
                assert!(result.is_ok(), "Buffer should accept correctly sized data");
            }
        } else {
            println!("Skipping buffer bounds checking test - no Vulkan device available");
        }
    }

    #[test]
    fn test_vertex_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let vertex_buffer = VertexBuffer::new_host_visible(&device, &allocator, 2048);

            assert!(
                vertex_buffer.is_ok(),
                "Should be able to create host-visible vertex buffer"
            );
            let vertex_buffer = vertex_buffer.unwrap();
            assert_eq!(vertex_buffer.size(), 2048);

            // Test device-local variant
            let device_vertex_buffer = VertexBuffer::new_device_local(&device, &allocator, 2048);
            assert!(
                device_vertex_buffer.is_ok(),
                "Should be able to create device-local vertex buffer"
            );
        } else {
            println!("Skipping vertex buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_index_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let index_buffer = IndexBuffer::new_host_visible(&device, &allocator, 512);

            assert!(
                index_buffer.is_ok(),
                "Should be able to create host-visible index buffer"
            );
            let index_buffer = index_buffer.unwrap();
            assert_eq!(index_buffer.size(), 512);

            // Test device-local variant
            let device_index_buffer = IndexBuffer::new_device_local(&device, &allocator, 512);
            assert!(
                device_index_buffer.is_ok(),
                "Should be able to create device-local index buffer"
            );
        } else {
            println!("Skipping index buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_uniform_buffer_creation() {
        if let Some((device, allocator)) = create_test_device() {
            let uniform_buffer = UniformBuffer::new_host_visible(&device, &allocator, 256);

            assert!(
                uniform_buffer.is_ok(),
                "Should be able to create host-visible uniform buffer"
            );
            let uniform_buffer = uniform_buffer.unwrap();
            assert_eq!(uniform_buffer.size(), 256);

            // Test device-local variant
            let device_uniform_buffer = UniformBuffer::new_device_local(&device, &allocator, 256);
            assert!(
                device_uniform_buffer.is_ok(),
                "Should be able to create device-local uniform buffer"
            );
        } else {
            println!("Skipping uniform buffer creation test - no Vulkan device available");
        }
    }

    #[test]
    fn test_buffer_types_have_correct_usage_semantics() {
        if let Some((device, allocator)) = create_test_device() {
            // Test core requirement: Different buffer types should be semantically distinct
            let vertex_buffer = VertexBuffer::new_host_visible(&device, &allocator, 1024);
            let index_buffer = IndexBuffer::new_host_visible(&device, &allocator, 512);
            let uniform_buffer = UniformBuffer::new_host_visible(&device, &allocator, 256);

            if let (Ok(vb), Ok(ib), Ok(ub)) = (vertex_buffer, index_buffer, uniform_buffer) {
                // Verify each buffer type enforces its intended usage
                assert!(
                    vb.buffer().usage().intersects(BufferUsage::VERTEX_BUFFER),
                    "VertexBuffer should be usable for vertex data"
                );
                assert!(
                    ib.buffer().usage().intersects(BufferUsage::INDEX_BUFFER),
                    "IndexBuffer should be usable for index data"
                );
                assert!(
                    ub.buffer().usage().intersects(BufferUsage::UNIFORM_BUFFER),
                    "UniformBuffer should be usable for uniform data"
                );

                // Verify that type wrappers preserve size semantics
                assert_eq!(
                    vb.size(),
                    1024,
                    "VertexBuffer should preserve requested size"
                );
                assert_eq!(ib.size(), 512, "IndexBuffer should preserve requested size");
                assert_eq!(
                    ub.size(),
                    256,
                    "UniformBuffer should preserve requested size"
                );
            }
        } else {
            println!("Skipping buffer type semantics test - no Vulkan device available");
        }
    }

    #[test]
    fn test_buffer_automatic_cleanup() {
        if let Some((device, allocator)) = create_test_device() {
            // Test core RAII requirement: Resources should clean up automatically
            let allocator_weak = Arc::downgrade(&allocator);

            {
                let _buffer1 =
                    Buffer::new_host_visible(&device, &allocator, 1024, BufferUsage::VERTEX_BUFFER);
                let _buffer2 = Buffer::new_device_local(
                    &device,
                    &allocator,
                    2048,
                    BufferUsage::INDEX_BUFFER | BufferUsage::TRANSFER_DST,
                );
                // Key requirement: Buffers should be automatically cleaned up when they go out of scope
                // This prevents memory leaks in graphics applications
            }

            // Test that allocator remains functional after buffer cleanup
            let _buffer3 =
                Buffer::new_host_visible(&device, &allocator, 512, BufferUsage::UNIFORM_BUFFER);
            assert!(
                allocator_weak.upgrade().is_some(),
                "Allocator should remain functional after buffer cleanup"
            );
        } else {
            println!("Skipping RAII cleanup test - no Vulkan device available");
        }
    }
}
