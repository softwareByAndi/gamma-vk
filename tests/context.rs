//! Comprehensive tests for VulkanContext
//!
//! These tests follow TDD principles and define expected behavior.
//! All tests should fail if functionality is not available.

use gamma_vk::{GammaVkError, VulkanContext};
use std::sync::Arc;
use vulkano::Version;
use vulkano::device::DeviceOwned;

// Helper function to skip tests when Vulkan is not available (e.g., in CI)
fn skip_if_no_vulkan() -> Option<VulkanContext> {
    match VulkanContext::new() {
        Ok(ctx) => Some(ctx),
        Err(GammaVkError::LibraryLoad(_)) => {
            eprintln!("Skipping test: Vulkan not available (expected in CI)");
            None
        }
        Err(e) => panic!("Unexpected error creating VulkanContext: {}", e),
    }
}

#[test]
fn context_creation_succeeds() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    // Context should be properly initialized
    // Don't make assumptions about specific layers/extensions
    // Just verify the accessors work without panicking
    let _ = context.enabled_layers();
    let _ = context.enabled_extensions();
    let _ = context.device();
    let _ = context.physical_device();

    // If we get here without panic, context is properly initialized
}

#[test]
fn device_accessor_returns_valid_device() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let device1 = context.device();
    let device2 = context.device();

    // Should return references to the same underlying device
    // Since we can't compare Arc pointers directly, verify they have same properties
    let phys1 = device1.physical_device();
    let phys2 = device2.physical_device();

    // Compare device names to ensure same physical device
    assert_eq!(
        phys1.properties().device_name,
        phys2.properties().device_name,
        "Device accessor should return same underlying device"
    );

    // Also verify the device UUIDs match (more reliable than name)
    assert_eq!(
        phys1.properties().device_uuid,
        phys2.properties().device_uuid,
        "Device UUIDs should match for same physical device"
    );

    // Device should be functional
    assert!(
        device1.api_version() >= Version::V1_0,
        "Device should support at least Vulkan 1.0"
    );
}

#[test]
fn physical_device_accessor_returns_valid_device() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let physical = context.physical_device();
    let props = physical.properties();

    // Basic validation
    assert!(
        !props.device_name.is_empty(),
        "Physical device should have a name"
    );
    assert!(
        props.api_version >= Version::V1_0,
        "Physical device should support at least Vulkan 1.0"
    );
}

#[test]
fn context_selects_graphics_capable_device() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let device = context.physical_device();

    // Verify we selected a graphics-capable device
    let queue_families = device.queue_family_properties();
    let has_graphics = queue_families.iter().any(|family| {
        family
            .queue_flags
            .intersects(vulkano::device::QueueFlags::GRAPHICS)
    });

    assert!(
        has_graphics,
        "Selected device must support graphics operations"
    );
}

#[test]
fn enabled_layers_accessor_works() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let layers = context.enabled_layers();

    // Verify we get a valid slice (even if empty)
    assert!(
        layers.is_empty() || layers.iter().all(|l| !l.is_empty()),
        "If layers are present, they should have valid names"
    );

    #[cfg(debug_assertions)]
    {
        // Could check for validation layers in debug, but don't require them
        if !layers.is_empty() {
            println!("Debug mode: Found {} layers: {:?}", layers.len(), layers);
        }
    }
}

#[test]
fn enabled_extensions_accessor_works() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let extensions = context.enabled_extensions();

    // On macOS, we expect either portability enumeration or fallback
    #[cfg(target_os = "macos")]
    {
        // We attempted portability, but it may have failed and fallen back
        println!(
            "MoltenVK extensions status: portability_enumeration={}",
            extensions.khr_portability_enumeration
        );
    }

    // Should at least have basic required extensions or explicitly none
    // The important thing is that extensions accessor works without panic
    println!("Enabled extensions: {:?}", extensions);
}

#[test]
fn context_cleanup_is_safe() {
    // Create and drop multiple contexts to ensure cleanup works
    for i in 0..3 {
        let Some(context) = skip_if_no_vulkan() else {
            return;
        };

        // Use the context
        let _device = context.device();
        let _physical = context.physical_device();

        println!("Context {} created and will be dropped", i);
        // Context dropped here
    }
    // Test passes if no crash/panic occurs
}

#[test]
fn context_device_relationship_is_correct() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let device = context.device();
    let physical = context.physical_device();

    // The logical device should be created from the physical device
    // Compare device properties to verify same physical device
    assert_eq!(
        device.physical_device().properties().device_uuid,
        physical.properties().device_uuid,
        "Logical device should be created from the context's physical device"
    );
}

#[test]
#[cfg(target_os = "macos")]
fn context_handles_moltenvk_on_macos() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    // On macOS, we should handle MoltenVK properly
    let extensions = context.enabled_extensions();

    // On macOS, context creation should succeed with either approach
    // The important thing is that it works, not which path it took

    // Log which path was taken for debugging
    if extensions.khr_portability_enumeration {
        println!("✓ MoltenVK initialized with portability extensions");
    } else {
        println!("✓ MoltenVK initialized with standard Vulkan");
    }

    // If we got here, MoltenVK was handled successfully
}

// Thread safety test
#[test]
fn context_is_thread_safe() {
    use std::thread;

    let Some(context) = skip_if_no_vulkan() else {
        return;
    };
    let context = Arc::new(context);

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let ctx = context.clone();
            thread::spawn(move || {
                // Each thread accesses the context
                let device = ctx.device();
                let physical = ctx.physical_device();

                // Verify consistency by comparing device UUIDs
                assert_eq!(
                    device.physical_device().properties().device_uuid,
                    physical.properties().device_uuid,
                    "Device relationships should be consistent across threads"
                );

                println!("Thread {} successfully accessed context", i);
            })
        })
        .collect();

    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

// Error type tests
#[test]
fn error_types_are_appropriate() {
    // Test that our error constructors work
    let init_error = GammaVkError::initialization("Test initialization error");

    // Test error display first (before moving)
    let error_string = init_error.to_string();
    assert!(error_string.contains("Initialization failed"));
    assert!(error_string.contains("Test initialization error"));

    // Then test the enum variant
    match init_error {
        GammaVkError::Initialization { message } => {
            assert_eq!(message, "Test initialization error");
        }
        _ => panic!("Wrong error type"),
    }
}

// Tests that require implementation changes
// These are commented out but show what we need to implement

#[test]
fn context_provides_graphics_queue() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let queue = context.graphics_queue();
    let family_index = context.graphics_queue_family_index();

    assert_eq!(queue.queue_family_index(), family_index);
}

#[test]
fn context_provides_memory_allocator() {
    let Some(context) = skip_if_no_vulkan() else {
        return;
    };

    let allocator1 = context.memory_allocator();
    let allocator2 = context.memory_allocator();

    // Should return references to the same allocator
    assert!(
        Arc::ptr_eq(&allocator1, &allocator2),
        "Memory allocator should be consistent across calls"
    );

    // Allocator should be associated with the same device
    assert!(
        Arc::ptr_eq(&allocator1.device(), &context.device()),
        "Allocator should use context's device"
    );
}

#[test]
fn context_builder_pattern_works() {
    // Try to create context with builder
    match VulkanContext::builder()
        .application_name("Test App")
        .application_version(1, 2, 3)
        .engine_name("Test Engine")
        .engine_version(4, 5, 6)
        .build()
    {
        Ok(context) => {
            // Verify context was created successfully
            let _ = context.device();
            let _ = context.graphics_queue();
            println!("Context created successfully with builder");
        }
        Err(GammaVkError::LibraryLoad(_)) => {
            eprintln!("Skipping test: Vulkan not available (expected in CI)");
        }
        Err(e) => {
            panic!("Unexpected error creating context with builder: {}", e);
        }
    }
}

#[test]
fn context_builder_with_minimal_config() {
    // Test builder with minimal configuration
    match VulkanContext::builder().build() {
        Ok(context) => {
            // Should work with just defaults
            let _ = context.device();
            println!("Context created with default builder settings");
        }
        Err(GammaVkError::LibraryLoad(_)) => {
            eprintln!("Skipping test: Vulkan not available (expected in CI)");
        }
        Err(e) => {
            panic!("Builder with defaults should work: {}", e);
        }
    }
}

/*
#[test]
fn context_prefers_discrete_gpu() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");

    let selected = context.physical_device();
    let all_devices: Vec<_> = context.instance
        .enumerate_physical_devices()
        .expect("Failed to enumerate devices");

    // If there's a discrete GPU, we should have selected it
    let has_discrete = all_devices.iter().any(|d| {
        d.properties().device_type == PhysicalDeviceType::DiscreteGpu
    });

    if has_discrete {
        assert_eq!(
            selected.properties().device_type,
            PhysicalDeviceType::DiscreteGpu,
            "Should prefer discrete GPU when available"
        );
    }
}

#[test]
#[cfg(debug_assertions)]
fn validation_layers_enabled_in_debug() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");

    let layers = context.enabled_layers();
    assert!(
        layers.iter().any(|l| l.contains("validation")),
        "Validation layers should be enabled in debug builds"
    );
}
*/
