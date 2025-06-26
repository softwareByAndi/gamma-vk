//! Comprehensive tests for buffer module
//!
//! These tests follow TDD principles to define expected buffer behavior.
//! Tests should fail when expected functionality is missing.

use gamma_vk::{
    buffer::{Buffer, IndexBuffer, UniformBuffer, VertexBuffer},
    GammaVkError, VulkanContext,
};
use std::sync::Arc;
use vulkano::{
    buffer::BufferUsage,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

// Helper to create test context with device and allocator
fn create_test_context() -> Option<(VulkanContext, Arc<StandardMemoryAllocator>)> {
    let context = match VulkanContext::new() {
        Ok(ctx) => ctx,
        Err(GammaVkError::LibraryLoad(_)) => {
            eprintln!("Skipping test: Vulkan not available (expected in CI)");
            return None;
        }
        Err(e) => panic!("Unexpected error creating VulkanContext: {}", e),
    };

    let allocator = Arc::new(StandardMemoryAllocator::new_default(context.device().clone()));
    Some((context, allocator))
}

// ========== Unit Tests - Core Buffer Functionality ==========

#[test]
fn test_buffer_creation_with_valid_size() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    );

    assert!(buffer.is_ok(), "Should create buffer with valid size");
    let buffer = buffer.unwrap();
    assert_eq!(buffer.size(), 1024);
}


#[test]
fn test_buffer_creation_with_zero_size_returns_error() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    // Clone the references we need for the closure
    let device = context.device();
    let allocator_clone = allocator.clone();

    // Catch the panic from Vulkano
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Buffer::new_host_visible(
            &device,
            &allocator_clone,
            0,
            BufferUsage::TRANSFER_DST,
        )
    }));

    // This test documents that zero-size buffers should fail
    // Current Vulkano implementation panics, which is not ideal
    assert!(
        result.is_err(),
        "Zero-size buffer creation should fail (currently panics in Vulkano)"
    );
    
    // TODO: The buffer module should validate size before calling Vulkano
    // to provide a better error message instead of a panic
}

#[test]
fn test_host_visible_buffer_is_cpu_accessible() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create host-visible buffer");

    assert!(
        buffer.is_host_visible(),
        "Host-visible buffer should report as CPU accessible"
    );

    // Should be able to write data
    let data = vec![42u8; 512];
    let result = buffer.write_data(&data);
    assert!(
        result.is_ok(),
        "Should be able to write to host-visible buffer"
    );
}

#[test]
fn test_device_local_buffer_is_not_cpu_accessible() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_device_local(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create device-local buffer");

    assert!(
        !buffer.is_host_visible(),
        "Device-local buffer should not be CPU accessible"
    );

    // Writing should fail
    let data = vec![42u8; 512];
    let result = buffer.write_data(&data);
    assert!(
        result.is_err(),
        "Should not be able to write directly to device-local buffer"
    );
}

#[test]
fn test_write_data_larger_than_buffer_fails() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create buffer");

    let data = vec![42u8; 2048]; // Larger than buffer
    let result = buffer.write_data(&data);

    assert!(result.is_err(), "Should fail when writing data larger than buffer");
    assert!(
        result.unwrap_err().to_string().contains("exceeds buffer size"),
        "Error message should explain size mismatch"
    );
}

#[test]
fn test_partial_buffer_write() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create buffer");

    let data = vec![42u8; 512]; // Half the buffer size
    let result = buffer.write_data(&data);

    assert!(result.is_ok(), "Should succeed when writing partial buffer data");
}

#[test]
fn test_custom_allocation_preferences_respected() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let custom_allocation = AllocationCreateInfo {
        memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
        ..Default::default()
    };

    let buffer = Buffer::new_custom(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
        custom_allocation,
    );

    assert!(
        buffer.is_ok(),
        "Should create buffer with custom allocation preferences"
    );
}

// ========== Type-Safe Buffer Wrapper Tests ==========

#[test]
fn test_vertex_buffer_has_correct_usage_flags() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let vertex_buffer = VertexBuffer::new_host_visible(&context.device(), &allocator, 1024)
        .expect("Failed to create vertex buffer");

    let usage = vertex_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::VERTEX_BUFFER),
        "Vertex buffer must have VERTEX_BUFFER usage flag"
    );
}

#[test]
fn test_vertex_buffer_device_local_includes_transfer_dst() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let vertex_buffer = VertexBuffer::new_device_local(&context.device(), &allocator, 1024)
        .expect("Failed to create device-local vertex buffer");

    let usage = vertex_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::VERTEX_BUFFER),
        "Must have VERTEX_BUFFER usage"
    );
    assert!(
        usage.contains(BufferUsage::TRANSFER_DST),
        "Device-local vertex buffer must have TRANSFER_DST for data uploads"
    );
}

#[test]
fn test_index_buffer_has_correct_usage_flags() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let index_buffer = IndexBuffer::new_host_visible(&context.device(), &allocator, 1024)
        .expect("Failed to create index buffer");

    let usage = index_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::INDEX_BUFFER),
        "Index buffer must have INDEX_BUFFER usage flag"
    );
}

#[test]
fn test_index_buffer_device_local_includes_transfer_dst() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let index_buffer = IndexBuffer::new_device_local(&context.device(), &allocator, 1024)
        .expect("Failed to create device-local index buffer");

    let usage = index_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::INDEX_BUFFER),
        "Must have INDEX_BUFFER usage"
    );
    assert!(
        usage.contains(BufferUsage::TRANSFER_DST),
        "Device-local index buffer must have TRANSFER_DST for data uploads"
    );
}

#[test]
fn test_uniform_buffer_has_correct_usage_flags() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let uniform_buffer = UniformBuffer::new_host_visible(&context.device(), &allocator, 1024)
        .expect("Failed to create uniform buffer");

    let usage = uniform_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::UNIFORM_BUFFER),
        "Uniform buffer must have UNIFORM_BUFFER usage flag"
    );
}

#[test]
fn test_uniform_buffer_device_local_includes_transfer_dst() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let uniform_buffer = UniformBuffer::new_device_local(&context.device(), &allocator, 1024)
        .expect("Failed to create device-local uniform buffer");

    let usage = uniform_buffer.buffer().usage();
    assert!(
        usage.contains(BufferUsage::UNIFORM_BUFFER),
        "Must have UNIFORM_BUFFER usage"
    );
    assert!(
        usage.contains(BufferUsage::TRANSFER_DST),
        "Device-local uniform buffer must have TRANSFER_DST for data uploads"
    );
}

// ========== Buffer Size Tests ==========

#[test]
fn test_vertex_buffer_size_accessible() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let size = 2048;
    let vertex_buffer = VertexBuffer::new_host_visible(&context.device(), &allocator, size)
        .expect("Failed to create vertex buffer");

    assert_eq!(vertex_buffer.size(), size, "Size should match requested value");
}

#[test]
fn test_index_buffer_size_accessible() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let size = 4096;
    let index_buffer = IndexBuffer::new_host_visible(&context.device(), &allocator, size)
        .expect("Failed to create index buffer");

    assert_eq!(index_buffer.size(), size, "Size should match requested value");
}

#[test]
fn test_uniform_buffer_size_accessible() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let size = 256;
    let uniform_buffer = UniformBuffer::new_host_visible(&context.device(), &allocator, size)
        .expect("Failed to create uniform buffer");

    assert_eq!(
        uniform_buffer.size(),
        size,
        "Size should match requested value"
    );
}

// ========== Staging Buffer Pattern Tests ==========

#[test]
fn test_staging_buffer_upload_placeholder_returns_error() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_device_local(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create device-local buffer");

    let data = vec![42u8; 512];
    let result = buffer.upload_via_staging(&context.device(), &allocator, &data);

    // Current implementation should return "not implemented" error
    assert!(result.is_err(), "Staging upload should fail in current implementation");
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("not yet implemented"),
        "Error should indicate feature is not implemented"
    );
}

// ========== Buffer Lifetime Tests ==========

#[test]
fn test_buffer_move_semantics() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    // FIXME - research if this test is implemented correctly

    let buffer1 = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create buffer");

    let size = buffer1.size();

    // Move buffer
    let buffer2 = buffer1;

    // Verify moved buffer works
    assert_eq!(buffer2.size(), size);

    // Original binding no longer accessible (compile-time check)
    // Uncommenting the next line should cause compilation error:
    // let _ = buffer1.size();
}

#[test]
fn test_multiple_buffers_independent_lifetime() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer1 = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create first buffer");

    let buffer2 = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        2048,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create second buffer");

    // Buffers should have independent sizes
    assert_eq!(buffer1.size(), 1024);
    assert_eq!(buffer2.size(), 2048);

    // Drop buffer1
    drop(buffer1);

    // buffer2 should still be valid
    assert_eq!(buffer2.size(), 2048);
}

// ========== Edge Case Tests ==========

#[test]
fn test_buffer_creation_with_odd_size() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    // Test various odd sizes that might cause alignment issues
    let odd_sizes = [17, 33, 127, 513, 1023];

    for size in odd_sizes {
        let buffer = Buffer::new_host_visible(
            &context.device(),
            &allocator,
            size,
            BufferUsage::TRANSFER_DST,
        );

        assert!(
            buffer.is_ok(),
            "Should handle odd size {} correctly",
            size
        );

        let buffer = buffer.unwrap();
        assert!(
            buffer.size() >= size,
            "Buffer size should be at least the requested size"
        );
    }
}

#[test]
fn test_null_data_write_handled() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::TRANSFER_DST,
    )
    .expect("Failed to create buffer");

    // Write empty slice
    let data: Vec<u8> = vec![];
    let result = buffer.write_data(&data);

    assert!(result.is_ok(), "Should handle empty data write gracefully");
}

#[test]
fn test_buffer_usage_validation() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    // Create buffer with multiple usage flags
    let buffer = Buffer::new_host_visible(
        &context.device(),
        &allocator,
        1024,
        BufferUsage::VERTEX_BUFFER | BufferUsage::TRANSFER_DST | BufferUsage::TRANSFER_SRC,
    );

    assert!(buffer.is_ok(), "Should allow valid usage flag combinations");

    let buffer = buffer.unwrap();
    let usage = buffer.usage();
    assert!(usage.contains(BufferUsage::VERTEX_BUFFER));
    assert!(usage.contains(BufferUsage::TRANSFER_DST));
    assert!(usage.contains(BufferUsage::TRANSFER_SRC));
}

// ========== Performance Characteristic Tests ==========

#[test]
fn test_buffer_creation_performance_reasonable() {
    let Some((context, allocator)) = create_test_context() else {
        return;
    };

    use std::time::Instant;

    let sizes = [1024, 1024 * 1024, 10 * 1024 * 1024]; // 1KB, 1MB, 10MB

    for size in sizes {
        let start = Instant::now();
        let buffer = Buffer::new_host_visible(
            &context.device(),
            &allocator,
            size,
            BufferUsage::TRANSFER_DST,
        );
        let duration = start.elapsed();

        assert!(buffer.is_ok(), "Buffer creation should succeed");

        // Buffer creation should be reasonably fast (< 100ms even for large buffers)
        assert!(
            duration.as_millis() < 100,
            "Buffer creation for size {} took too long: {:?}",
            size,
            duration
        );
    }
}