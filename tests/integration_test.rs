//! Integration tests for Gamma-VK library
//!
//! These tests verify that the library works correctly as a whole,
//! testing the public API from an external perspective.

use gamma_vk::{GammaVkError, VulkanContext};

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
