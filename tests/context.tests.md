# VulkanContext Test Plan

This document outlines comprehensive test cases for `VulkanContext` following TDD principles. These tests define the expected behavior and will guide implementation improvements.

## Philosophy

The `VulkanContext` is the foundation of the entire graphics engine. Its tests should:
- Verify correct initialization in various environments
- Ensure robust error handling for missing/incompatible hardware
- Validate resource lifecycle management
- Guide API design for clarity and safety
- Cover platform-specific quirks (especially MoltenVK on macOS)

## Important: Test Success vs Failure Patterns

### Tests MUST FAIL when:
- **Any error occurs** - Including graceful errors like missing Vulkan drivers
- **Expected functionality is not available** - If we can't create a context, the test fails
- **Resources are not properly managed** - Leaks, improper cleanup, etc.

### Tests only PASS when:
- **All expected functionality works correctly**
- **Resources are properly managed**
- **No errors occur during execution**

**Note**: If Vulkan is not available in CI or certain environments, tests should fail. Use CI configuration to skip Vulkan tests rather than making tests pass when functionality is missing.

## Test Categories

### 1. Initialization Tests

#### ✅ Basic Creation Success [IMPLEMENTED]
```rust
#[test]
fn context_creation_succeeds() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // Verify context is properly initialized
    assert!(!context.enabled_layers().is_empty() || 
           !context.enabled_extensions().is_empty());
}
```
- **Purpose**: Verify successful initialization
- **Fails when**: Vulkan drivers missing, initialization fails
- **Status**: ✅ Test passes on macOS with MoltenVK

#### ❌ MoltenVK Fallback Verification
```rust
#[test]
fn context_uses_correct_initialization_path() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // Verify which initialization path was used
    let extensions = context.enabled_extensions();
    
    #[cfg(target_os = "macos")]
    {
        // On macOS, we should have portability extensions
        assert!(extensions.khr_portability_enumeration,
                "MoltenVK portability extensions should be enabled on macOS");
    }
}
```

#### ❌ Physical Device Selection
```rust
#[test]
fn context_selects_appropriate_physical_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let device = context.physical_device();
    let props = device.properties();
    
    // Verify we selected a graphics-capable device
    let queue_families = device.queue_family_properties();
    let has_graphics = queue_families.iter().any(|family| {
        family.queue_flags.intersects(QueueFlags::GRAPHICS)
    });
    
    assert!(has_graphics, "Selected device must support graphics operations");
    
    // Verify device name is reasonable
    assert!(!props.device_name.is_empty());
}
```

### 2. Device Selection Tests

#### ❌ Discrete GPU Preference
```rust
#[test]
fn context_prefers_discrete_gpu_when_available() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let selected_device = context.physical_device();
    let all_devices: Vec<_> = context.instance
        .enumerate_physical_devices()
        .expect("Failed to enumerate devices");
    
    // If there's a discrete GPU available, we should have selected it
    let has_discrete = all_devices.iter().any(|d| {
        d.properties().device_type == PhysicalDeviceType::DiscreteGpu
    });
    
    if has_discrete {
        assert_eq!(
            selected_device.properties().device_type,
            PhysicalDeviceType::DiscreteGpu,
            "Should select discrete GPU when available"
        );
    }
}
```

#### ❌ Device Scoring Algorithm
```rust
#[test]
fn context_scores_devices_correctly() {
    // This test will fail until scoring is implemented
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // Should expose device scoring for testing
    let scores = context.score_all_devices();
    
    // Discrete GPUs should score higher than integrated
    for (device, score) in scores {
        let device_type = device.properties().device_type;
        match device_type {
            PhysicalDeviceType::DiscreteGpu => assert!(score >= 1000),
            PhysicalDeviceType::IntegratedGpu => assert!(score >= 100),
            PhysicalDeviceType::VirtualGpu => assert!(score >= 10),
            _ => assert!(score >= 1),
        }
    }
}
```

### 3. Resource Management Tests

#### ❌ RAII Cleanup
```rust
#[test]
fn context_cleanup_is_deterministic() {
    // Create context in a scope
    {
        let context = VulkanContext::new()
            .expect("Failed to create VulkanContext");
        
        // Get device reference count before drop
        let device = context.device();
        let initial_count = Arc::strong_count(&device);
        assert!(initial_count >= 2); // Context + our reference
    }
    
    // Context dropped, verify cleanup happened
    // This test needs implementation support to verify
}
```

#### ❌ Thread Safety
```rust
#[test]
fn context_is_thread_safe() {
    use std::sync::Arc;
    use std::thread;
    
    let context = Arc::new(VulkanContext::new()
        .expect("Failed to create VulkanContext"));
    
    let handles: Vec<_> = (0..4).map(|i| {
        let ctx = context.clone();
        thread::spawn(move || {
            // Each thread accesses the context
            let device = ctx.device();
            let physical = ctx.physical_device();
            assert_eq!(device.physical_device(), &physical);
            println!("Thread {} accessed context", i);
        })
    }).collect();
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
```

### 4. Configuration Tests

#### ❌ Builder Pattern API
```rust
#[test]
fn context_builder_creates_configured_context() {
    let context = VulkanContext::builder()
        .application_name("Test App")
        .application_version(1, 0, 0)
        .engine_name("Gamma-VK")
        .prefer_device_type(PhysicalDeviceType::DiscreteGpu)
        .require_graphics_queue()
        .build()
        .expect("Failed to create configured context");
    
    // Verify configuration was applied
    // This will fail until builder is implemented
}
```

#### ❌ Validation Layers in Debug
```rust
#[test]
#[cfg(debug_assertions)]
fn context_enables_validation_in_debug() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let layers = context.enabled_layers();
    assert!(
        layers.iter().any(|l| l.contains("VK_LAYER_KHRONOS_validation")),
        "Validation layers should be enabled in debug builds"
    );
}
```

### 5. Query and Accessor Tests

#### ✅ Device Accessor [IMPLEMENTED]
```rust
#[test]
fn device_accessor_returns_valid_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let device1 = context.device();
    let device2 = context.device();
    
    // Should return same device instance
    assert!(Arc::ptr_eq(&device1, &device2));
    
    // Device should be functional
    assert!(device1.api_version() >= Version::V1_0);
}
```
- **Status**: ✅ Test passes

#### ✅ Physical Device Accessor [IMPLEMENTED]
```rust
#[test]
fn physical_device_accessor_returns_valid_device() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let physical = context.physical_device();
    let props = physical.properties();
    
    // Basic validation
    assert!(!props.device_name.is_empty());
    assert!(props.api_version >= Version::V1_0);
}
```
- **Status**: ✅ Test passes

#### ✅ Enabled Layers Accessor [IMPLEMENTED]
- **Status**: ✅ Test passes

#### ✅ Enabled Extensions Accessor [IMPLEMENTED]
- **Status**: ✅ Test passes

#### ❌ Graphics Queue Accessor
```rust
#[test]
fn context_provides_graphics_queue() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    let queue = context.graphics_queue();
    let family_index = context.graphics_queue_family_index();
    
    assert_eq!(queue.queue_family_index(), family_index);
    
    // Verify it's actually a graphics queue
    let family_props = context.physical_device()
        .queue_family_properties()[family_index as usize];
    assert!(family_props.queue_flags.intersects(QueueFlags::GRAPHICS));
}
```

### 6. Error Cases Tests

#### ❌ Error Types Are Specific
```rust
#[test]
fn initialization_errors_have_correct_types() {
    // This would need mock/injection to test different failure modes
    // Each failure mode should produce a specific error variant
    
    // Example: Test no devices error
    let result = create_context_with_no_devices();
    match result {
        Err(GammaVkError::Initialization { message }) => {
            assert!(message.contains("No physical devices"));
        }
        _ => panic!("Expected Initialization error for no devices"),
    }
}
```

#### ❌ Error Messages Are Actionable
```rust
#[test]
fn error_messages_provide_useful_context() {
    // Force various errors and verify messages
    let error = GammaVkError::initialization("No compatible GPU found");
    let message = error.to_string();
    
    // Message should be clear and actionable
    assert!(message.len() > 20, "Error message too short");
    assert!(message.contains("GPU"), "Should mention GPU");
}
```

### 7. Integration Tests

#### ❌ Full Initialization Flow
```rust
#[test]
fn complete_initialization_flow_works() {
    let context = VulkanContext::new()
        .expect("Failed to create VulkanContext");
    
    // Verify all components are properly initialized
    let device = context.device();
    let physical = context.physical_device();
    let instance = &context.instance;
    
    // Verify relationships
    assert_eq!(device.physical_device(), &physical);
    assert_eq!(physical.instance(), instance);
    
    // Verify we can create resources
    let buffer_info = BufferCreateInfo {
        size: 1024,
        usage: BufferUsage::VERTEX_BUFFER,
        ..Default::default()
    };
    
    let buffer = Buffer::new(device.clone(), buffer_info)
        .expect("Should be able to create buffer with initialized context");
}
```

## Test Implementation Status

### Completed Tests (11/30+)
- ✅ `context_creation_succeeds` - Basic initialization works
- ✅ `device_accessor_returns_valid_device` - Device accessor works correctly
- ✅ `physical_device_accessor_returns_valid_device` - Physical device accessor works
- ✅ `context_selects_graphics_capable_device` - Verifies graphics queue selection
- ✅ `enabled_layers_accessor_works` - Layer query works
- ✅ `enabled_extensions_accessor_works` - Extension query works
- ✅ `context_cleanup_is_safe` - Multiple context creation/destruction works
- ✅ `context_device_relationship_is_correct` - Device relationships are valid
- ✅ `context_handles_moltenvk_on_macos` - MoltenVK handling verified
- ✅ `context_is_thread_safe` - Thread safety verified
- ✅ `error_types_are_appropriate` - Error construction works

### Priority Implementation Tasks

#### Phase 1: Core Missing Features
1. ❌ Graphics queue accessor method
2. ❌ Better device selection (prefer discrete GPU)
3. ❌ Device scoring system

#### Phase 2: API Improvements
1. ❌ Builder pattern for configuration
2. ❌ Validation layer support
3. ❌ Custom extension configuration

#### Phase 3: Advanced Features
1. ❌ Multiple queue family support
2. ❌ Device feature requirements
3. ❌ Performance monitoring

## Test Helpers

```rust
/// Run test only if Vulkan is available
/// Usage: #[cfg_attr(not(feature = "ci"), test)]
fn requires_vulkan() {
    // Tests that require Vulkan should just fail if it's not available
    // Use build configuration to skip in CI, not runtime checks
}

/// Helper for tests that need specific device types
fn find_device_of_type(device_type: PhysicalDeviceType) -> Option<Arc<PhysicalDevice>> {
    let instance = Instance::new(/* ... */)?;
    instance.enumerate_physical_devices()
        .ok()?
        .find(|d| d.properties().device_type == device_type)
}
```

## CI Configuration

For CI environments without Vulkan, use Cargo features or environment variables to skip tests:

```toml
# Cargo.toml
[features]
ci = []  # Skip Vulkan tests in CI

# Run tests in CI with:
# cargo test --features ci
```

Or use `#[ignore]` attribute with a CI check:

```rust
#[test]
#[cfg_attr(env!("CI"), ignore)]
fn test_requiring_vulkan() {
    // Test that needs real Vulkan
}
```

## Key Principles

1. **Tests fail when functionality doesn't work** - No masking of failures
2. **Use CI configuration to skip tests** - Not runtime checks that pass
3. **Each test verifies real behavior** - Not just that errors are graceful
4. **Test failure drives implementation** - Failed tests show what needs fixing
5. **Clear failure messages** - When test fails, it should be obvious why

## Notes

- Tests marked ✅ can be implemented with current code
- Tests marked ❌ require implementation changes
- All tests should fail if VulkanContext cannot be created
- Use platform configuration to handle environment differences, not test logic