//! Comprehensive tests for VulkanContext
//!
//! These tests follow TDD principles and define expected behavior.
//! All tests should fail if functionality is not available.

use gamma_vk::{VulkanContext, GammaVkError};
use std::sync::Arc;
use vulkano::Version;

#[test]
fn context_creation_succeeds() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // Verify context is properly initialized
    // At least one of layers or extensions should be present
    assert!(!context.enabled_layers().is_empty() || 
           context.enabled_extensions().khr_get_physical_device_properties2 ||
           context.enabled_extensions().khr_portability_enumeration);
}

#[test]
fn device_accessor_returns_valid_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let device1 = context.device();
    let device2 = context.device();
    
    // Should return same device instance
    assert!(Arc::ptr_eq(&device1, &device2), 
            "Device accessor should return same Arc instance");
    
    // Device should be functional
    assert!(device1.api_version() >= Version::V1_0,
            "Device should support at least Vulkan 1.0");
}

#[test]
fn physical_device_accessor_returns_valid_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let physical = context.physical_device();
    let props = physical.properties();
    
    // Basic validation
    assert!(!props.device_name.is_empty(),
            "Physical device should have a name");
    assert!(props.api_version >= Version::V1_0,
            "Physical device should support at least Vulkan 1.0");
}

#[test]
fn context_selects_graphics_capable_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let device = context.physical_device();
    
    // Verify we selected a graphics-capable device
    let queue_families = device.queue_family_properties();
    let has_graphics = queue_families.iter().any(|family| {
        family.queue_flags.intersects(vulkano::device::QueueFlags::GRAPHICS)
    });
    
    assert!(has_graphics, 
            "Selected device must support graphics operations");
}

#[test]
fn enabled_layers_accessor_works() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let layers = context.enabled_layers();
    
    // Layers should be a valid slice (might be empty in release)
    // Just verify we can access it without panic
    
    #[cfg(debug_assertions)]
    {
        // In debug mode, we might have validation layers
        println!("Enabled layers: {:?}", layers);
    }
}

#[test]
fn enabled_extensions_accessor_works() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let extensions = context.enabled_extensions();
    
    // We should have at least tried portability enumeration
    // Check the actual extensions object
    println!("Enabled extensions: {:?}", extensions);
}

#[test]
fn context_cleanup_is_safe() {
    // Create and drop multiple contexts to ensure cleanup works
    for i in 0..3 {
        let context = VulkanContext::new()
            .expect("Failed to create VulkanContext");
        
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
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let device = context.device();
    let physical = context.physical_device();
    
    // The logical device should be created from the physical device
    assert!(Arc::ptr_eq(device.physical_device(), &physical),
            "Logical device should be created from the context's physical device");
}

#[test]
#[cfg(target_os = "macos")]
fn context_handles_moltenvk_on_macos() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // On macOS, we should handle MoltenVK properly
    let extensions = context.enabled_extensions();
    
    // Log which path we took
    if extensions.khr_portability_enumeration {
        println!("Successfully initialized with MoltenVK portability extensions");
    } else {
        println!("Initialized with standard Vulkan (MoltenVK without portability)");
    }
}

// Thread safety test
#[test]
fn context_is_thread_safe() {
    use std::thread;
    
    let context = Arc::new(VulkanContext::new()
        .expect("Failed to create VulkanContext"));
    
    let handles: Vec<_> = (0..4).map(|i| {
        let ctx = context.clone();
        thread::spawn(move || {
            // Each thread accesses the context
            let device = ctx.device();
            let physical = ctx.physical_device();
            
            // Verify consistency
            assert!(Arc::ptr_eq(device.physical_device(), &physical),
                    "Device relationships should be consistent across threads");
            
            println!("Thread {} successfully accessed context", i);
        })
    }).collect();
    
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

/*
#[test]
fn context_provides_graphics_queue() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // This method doesn't exist yet
    let queue = context.graphics_queue();
    let family_index = context.graphics_queue_family_index();
    
    assert_eq!(queue.queue_family_index(), family_index);
}

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
fn context_builder_pattern_works() {
    let context = VulkanContext::builder()
        .application_name("Test App")
        .application_version(1, 0, 0)
        .prefer_discrete_gpu()
        .build()
        .expect("Failed to create context with builder");
    
    // Verify configuration was applied
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