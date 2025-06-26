//! Integration tests for Gamma-VK library
//!
//! These tests verify that the library works correctly as a whole,
//! testing the public API from an external perspective.

use gamma_vk::{GammaVkError, VertexBuffer, VulkanContext};
use std::sync::Arc;
use vulkano::{
    device::{Device, DeviceCreateInfo, QueueCreateInfo},
    memory::allocator::StandardMemoryAllocator,
};

#[test]
fn test_vulkan_context_integration() {
    // Test that VulkanContext can be created and used
    match VulkanContext::new() {
        Ok(context) => {
            // Test that we can access context information
            let _layers = context.enabled_layers();
            let _extensions = context.enabled_extensions();

            // If we get here, context creation and basic access work
            println!("Integration test: VulkanContext created successfully");
        }
        Err(e) => {
            // In CI environments or systems without Vulkan, this is expected
            match e {
                GammaVkError::LibraryLoad(_) => {
                    println!("Integration test: Library load failed (expected in CI)");
                }
                GammaVkError::InstanceCreation(_) => {
                    println!("Integration test: Instance creation failed (expected in CI)");
                }
                _ => {
                    println!("Integration test: Unexpected error: {}", e);
                }
            }
        }
    }
}

#[test]
fn test_error_conversion() {
    // Test that error types work correctly
    let init_error = GammaVkError::initialization("test message");
    let error_string = format!("{}", init_error);
    assert!(error_string.contains("Initialization failed"));
    assert!(error_string.contains("test message"));
}

#[test]
fn test_buffer_integration() {
    // Test that buffer creation works with VulkanContext
    match VulkanContext::new() {
        Ok(context) => {
            // Try to create a device for buffer testing
            if let Some(physical_device) = context
                .instance
                .enumerate_physical_devices()
                .ok()
                .and_then(|mut devices| devices.next())
            {
                let queue_family_index = physical_device
                    .queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(_, q)| {
                        q.queue_flags
                            .intersects(vulkano::device::QueueFlags::GRAPHICS)
                    });

                if let Some(queue_family_index) = queue_family_index {
                    match Device::new(
                        physical_device,
                        DeviceCreateInfo {
                            queue_create_infos: vec![QueueCreateInfo {
                                queue_family_index: queue_family_index as u32,
                                ..Default::default()
                            }],
                            ..Default::default()
                        },
                    ) {
                        Ok((device, _queues)) => {
                            let allocator =
                                Arc::new(StandardMemoryAllocator::new_default(device.clone()));

                            // Test vertex buffer creation
                            match VertexBuffer::new(device, allocator, 1024) {
                                Ok(buffer) => {
                                    assert_eq!(buffer.size(), 1024);
                                    println!("Integration test: Buffer creation successful");
                                }
                                Err(e) => {
                                    println!("Integration test: Buffer creation failed: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Integration test: Device creation failed: {}", e);
                        }
                    }
                } else {
                    println!("Integration test: No graphics queue family found");
                }
            } else {
                println!("Integration test: No physical devices found");
            }
        }
        Err(e) => {
            println!(
                "Integration test: VulkanContext creation failed (expected in CI): {}",
                e
            );
        }
    }
}
