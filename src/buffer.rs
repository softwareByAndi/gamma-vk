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
///
/// # Move Semantics
///
/// Buffers implement move semantics but not copy semantics, preventing
/// accidental resource duplication:
///
/// ```compile_fail
/// # use gamma_vk::buffer::Buffer;
/// # use std::sync::Arc;
/// # use vulkano::buffer::BufferUsage;
/// # use vulkano::memory::allocator::StandardMemoryAllocator;
/// # fn test_copy(device: &Arc<vulkano::device::Device>, allocator: &Arc<StandardMemoryAllocator>) {
/// let buffer1 = Buffer::new_host_visible(device, allocator, 1024, BufferUsage::TRANSFER_DST).unwrap();
/// let buffer2 = buffer1; // OK: move
/// let buffer3 = buffer1; // Error: use of moved value
/// # }
/// ```
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
        // Validate size per Vulkan spec VUID-VkBufferCreateInfo-size-00912
        if size == 0 {
            return Err(GammaVkError::buffer_creation(
                "Buffer size must be greater than 0".to_string(),
            ));
        }

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
        // Validate size per Vulkan spec VUID-VkBufferCreateInfo-size-00912
        if size == 0 {
            return Err(GammaVkError::buffer_creation(
                "Buffer size must be greater than 0".to_string(),
            ));
        }

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
        // Validate size per Vulkan spec VUID-VkBufferCreateInfo-size-00912
        if size == 0 {
            return Err(GammaVkError::buffer_creation(
                "Buffer size must be greater than 0".to_string(),
            ));
        }

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
    ///
    /// This method checks if the buffer's memory can be accessed from the CPU.
    /// Host-visible memory has the VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT set.
    pub fn is_host_visible(&self) -> bool {
        // In Vulkano 0.35, the most reliable way to check host visibility
        // is to attempt to obtain a write lock. If the memory is not host-visible,
        // this will fail. This approach is more accurate than trying to inspect
        // memory properties directly, which Vulkano doesn't expose in a straightforward way.
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
