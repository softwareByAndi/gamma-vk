# Test-Driven Development Patterns for Gamma-VK

This guide establishes comprehensive testing patterns for the Gamma-VK Vulkan graphics engine. It emphasizes Test-Driven Development (TDD) principles where tests drive implementation, not the other way around.

## Core TDD Philosophy

### The TDD Cycle
1. **Red**: Write a failing test that defines desired behavior
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve code quality while keeping tests green

### Critical Mindset
Before writing any test, ask yourself:
- What behavior am I specifying, not just what code am I testing?
- What role does this test play in the development cycle?
- What use cases and edge cases should be covered?
- Is the codebase ready for this test, or would it require too much scaffolding?
- Will this test guide future refactoring effectively?

## Test Categories and Their Roles

### 1. Unit Tests
**Purpose**: Verify individual components in isolation  
**When to Write**: When you can test behavior without external dependencies  
**When NOT to Write**: When isolation requires excessive mocking that obscures intent

```rust
// GOOD: Tests behavior, not implementation
#[test]
fn buffer_size_calculation_handles_alignment() {
    let size = calculate_aligned_size(100, 16);
    assert_eq!(size, 112); // Next multiple of 16
}

// BAD: Tests implementation details
#[test]
fn buffer_struct_has_size_field() {
    // Don't test private fields exist
}
```

### 2. Integration Tests
**Purpose**: Verify component interactions and contracts  
**When to Write**: When testing real-world usage patterns  
**When NOT to Write**: When dependencies aren't stable yet

```rust
// GOOD: Tests actual graphics pipeline behavior
#[test]
fn render_pipeline_draws_triangle() {
    let context = test_helpers::create_test_context();
    let pipeline = Pipeline::new()
        .with_vertex_shader("shaders/triangle.vert")
        .with_fragment_shader("shaders/basic.frag")
        .build(&context)?;
    
    let output = pipeline.render_frame();
    assert!(output.pixel_at(256, 256).is_red());
}
```

### 3. Acceptance Tests
**Purpose**: Verify end-user visible behavior  
**When to Write**: For each user story or feature  
**When NOT to Write**: For internal implementation details

```rust
// Tests what users actually care about
#[test]
fn application_renders_at_60_fps() {
    let app = Application::new();
    let frame_times = measure_frame_times(&app, 100);
    
    assert!(frame_times.percentile_95() < Duration::from_millis(16));
}
```

## Graphics-Specific Testing Patterns

### Resource Lifecycle Tests
```rust
#[test]
fn gpu_resources_cleaned_up_on_drop() {
    let initial_memory = query_gpu_memory_usage();
    
    {
        let _buffer = Buffer::new(1024 * 1024); // 1MB
        assert!(query_gpu_memory_usage() > initial_memory);
    }
    
    // Buffer dropped, memory should be freed
    assert_eq!(query_gpu_memory_usage(), initial_memory);
}
```

### Shader Testing
```rust
mod shader_tests {
    // Test shader compilation separately from execution
    #[test]
    fn vertex_shader_compiles() {
        let spirv = compile_glsl_to_spirv(VERTEX_SHADER_SOURCE);
        validate_spirv(&spirv).expect("Invalid SPIR-V");
    }
    
    // Test shader behavior with known inputs/outputs
    #[test]
    fn vertex_shader_transforms_correctly() {
        let shader = TestShaderRunner::new(VERTEX_SHADER);
        let input = vertex_data();
        let output = shader.run(input);
        
        assert_relative_eq!(output.position, expected_position());
    }
}
```

### Vulkan API Mocking Strategy
```rust
// When to mock: Testing error handling paths
#[test]
fn pipeline_creation_handles_device_lost() {
    let mock_device = MockDevice::new()
        .with_error(vk::Result::ERROR_DEVICE_LOST);
    
    let result = Pipeline::new(&mock_device);
    assert!(matches!(result, Err(GammaVkError::DeviceLost)));
}

// When NOT to mock: Testing actual rendering
// Use real Vulkan with test fixtures instead
```

## Edge Case Identification

### Systematic Edge Case Discovery
1. **Boundary Analysis**: Min/max values, empty collections, single elements
2. **Error Conditions**: Invalid inputs, resource exhaustion, device loss
3. **Concurrency**: Race conditions, synchronization edge cases
4. **Platform Differences**: MoltenVK quirks, driver variations

### Example: Comprehensive Buffer Tests
```rust
mod buffer_edge_cases {
    #[test]
    fn zero_sized_buffer_rejected() {
        let result = Buffer::new(0);
        assert!(matches!(result, Err(GammaVkError::InvalidSize(_))));
    }
    
    #[test]
    fn maximum_buffer_size_handled() {
        let max_size = query_max_buffer_size();
        let result = Buffer::new(max_size + 1);
        assert!(result.is_err());
    }
    
    #[test]
    fn buffer_alignment_requirements_met() {
        // Vulkan requires specific alignments
        let buffer = Buffer::new(17); // Odd size
        assert_eq!(buffer.allocated_size() % 16, 0);
    }
}
```

## When NOT to Write Tests

### 1. Premature Abstraction Tests
```rust
// DON'T: Test for future flexibility that doesn't exist
#[test]
fn renderer_supports_multiple_backends() {
    // Don't test multi-backend support if only Vulkan exists
}
```

### 2. Implementation Detail Tests
```rust
// DON'T: Test private methods or internal state
#[test]
fn buffer_internal_handle_is_valid() {
    // Users don't care about internal handles
}
```

### 3. Scaffolding-Heavy Tests
```rust
// DON'T: Write tests requiring excessive setup
#[test]
fn full_deferred_rendering_pipeline() {
    // If this needs 500 lines of setup, the codebase isn't ready
}
```

## Test Quality Metrics

### Good Tests Are:
- **Focused**: Test one behavior per test
- **Fast**: Milliseconds, not seconds
- **Deterministic**: Same result every time
- **Isolated**: No shared state between tests
- **Descriptive**: Test name explains the "what" and "why"

### Test Naming Convention
```rust
// Pattern: test_[unit]_[scenario]_[expected_result]
#[test]
fn test_buffer_creation_with_zero_size_returns_error() { }

#[test]
fn test_pipeline_compilation_with_invalid_shader_fails_gracefully() { }
```

## Refactoring to Match Tests

### The TDD Promise
Tests written first become the specification. Code should be refactored to:
1. Make tests pass with minimal implementation
2. Make tests clearer by improving APIs
3. Make tests faster by optimizing critical paths

### Example: API Evolution Through Tests
```rust
// Test drives API design
#[test]
fn intuitive_buffer_creation() {
    // This test shows desired API
    let buffer = Buffer::typed::<Vertex>()
        .with_capacity(1000)
        .gpu_only()
        .build()?;
    
    // Implementation refactored to support this API
}
```

## Platform and Environment Considerations

### Handling Missing Vulkan
```rust
fn skip_if_no_vulkan() -> Option<TestContext> {
    match TestContext::new() {
        Ok(ctx) => Some(ctx),
        Err(e) if e.is_vulkan_missing() => {
            eprintln!("Skipping test: {}", e);
            None
        }
        Err(e) => panic!("Unexpected error: {}", e),
    }
}

#[test]
fn test_requiring_vulkan() {
    let Some(ctx) = skip_if_no_vulkan() else { return };
    // Test implementation
}
```

### CI-Friendly Patterns
```rust
// Use environment variables for CI detection
fn is_ci() -> bool {
    std::env::var("CI").is_ok()
}

#[test]
fn test_with_visual_output() {
    if is_ci() {
        // Run headless validation only
        validate_render_output();
    } else {
        // Show window for manual verification
        display_render_output();
    }
}
```

## Test Organization

### Recommended Structure
```
tests/
├── unit/
│   ├── buffers.rs      # Isolated buffer tests
│   ├── shaders.rs      # Shader compilation tests
│   └── math.rs         # Pure computation tests
├── integration/
│   ├── rendering.rs    # Full render pipeline tests
│   ├── resources.rs    # Resource lifecycle tests
│   └── windowing.rs    # Window system integration
└── fixtures/
    ├── shaders/        # Test shader files
    └── models/         # Test geometry data
```

### Test Helpers and Builders
```rust
// Common test utilities
pub mod test_helpers {
    pub struct TestContextBuilder {
        validation: bool,
        device_extensions: Vec<&'static str>,
    }
    
    impl TestContextBuilder {
        pub fn minimal() -> Self {
            Self {
                validation: false,
                device_extensions: vec![],
            }
        }
        
        pub fn with_validation() -> Self {
            Self {
                validation: true,
                ..Self::minimal()
            }
        }
    }
}
```

## Anti-Patterns to Avoid

### 1. Testing the Framework
```rust
// DON'T: Test that Vulkan works correctly
#[test]
fn vulkan_creates_valid_devices() {
    // Vulkan's correctness is not our responsibility
}
```

### 2. Brittle Time-Based Tests
```rust
// DON'T: Rely on wall-clock time
#[test]
fn operation_completes_in_one_second() {
    let start = Instant::now();
    operation();
    assert!(start.elapsed() < Duration::from_secs(1)); // Flaky!
}

// DO: Test logical time progression
#[test]
fn operation_completes_within_frame_budget() {
    let mut sim_time = SimulatedTime::new();
    operation(&mut sim_time);
    assert!(sim_time.elapsed() < FRAME_TIME);
}
```

### 3. God Object Tests
```rust
// DON'T: Test everything in one test
#[test]
fn test_entire_application() {
    // 500 lines of assertions...
}

// DO: Focused, single-purpose tests
#[test]
fn window_resizing_updates_swapchain() { }

#[test] 
fn window_minimizing_pauses_rendering() { }
```

## Summary

Effective TDD in Gamma-VK requires:
1. **Think First**: What behavior are you specifying?
2. **Test at the Right Level**: Unit for algorithms, integration for workflows
3. **Embrace Failure**: Red tests guide implementation
4. **Refactor Ruthlessly**: Let tests drive better design
5. **Know When to Stop**: Don't test what isn't ready

Remember: Tests are not about code coverage, they're about confidence in behavior. Write tests that give you confidence the system does what users need, and let those tests guide your implementation toward that goal.

## Original Implementation Notes

### File-Based Resource Testing
**Principle**: Tests should fail when expected resources are missing  
**Exception**: Only skip when testing optional/missing resource handling

### Correct Pattern for Required Resources
```rust
#[test]
fn test_shader_file_loading() {
    let context = VulkanContext::new().expect("Failed to create context");
    
    // File MUST exist - test fails if not
    let shader = ShaderModule::from_spirv_file(&context.device(), "shaders/test.spv")
        .expect("Required test shader file missing");
    
    // Test the shader functionality
    assert!(shader.is_valid());
}
```

### Pattern for Testing Missing Resource Handling
```rust
#[test] 
fn test_missing_shader_error() {
    let context = VulkanContext::new().expect("Failed to create context");
    
    // Test that missing files produce correct error
    let result = ShaderModule::from_spirv_file(&context.device(), "nonexistent.spv");
    assert!(result.is_err());
    
    match result.unwrap_err() {
        GammaVkError::ShaderCompilation { message } => {
            assert!(message.contains("Failed to read shader file"));
        }
        _ => panic!("Wrong error type for missing file"),
    }
}
```

### Test Organization
```rust
// Group related tests
mod shader_tests {
    // Unit tests that don't need files
    #[test]
    fn test_spirv_validation() { ... }
    
    // Integration tests that need files
    #[test]
    fn test_file_loading() {
        // Check file exists first
    }
}
```

### Key Principles
1. **Never assume resources exist** - Check first
2. **Print informative skip messages** - Help debugging
3. **Test core logic separately** - From file I/O
4. **Use test fixtures** - For predictable testing

### Example: Robust Test Structure
```rust
fn create_test_device() -> Option<(Arc<Device>, Arc<Allocator>)> {
    // Returns None if Vulkan not available
    // Allows tests to skip gracefully
}

#[test]
fn test_buffer_creation() {
    if let Some((device, allocator)) = create_test_device() {
        // Run test
    } else {
        println!("Skipping test - no Vulkan device");
    }
}
```