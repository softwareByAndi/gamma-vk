//! Comprehensive tests for the ShaderModule functionality
//!
//! These tests follow TDD principles and verify shader loading, validation,
//! and resource management behaviors.

use gamma_vk::{GammaVkError, ShaderModule, VulkanContext};

// Test helper functions
mod helpers {
    use super::*;
    
    /// Creates a test Vulkan context if available
    pub fn create_test_context() -> Option<VulkanContext> {
        match VulkanContext::new() {
            Ok(ctx) => Some(ctx),
            Err(e) => {
                eprintln!("Skipping test - Vulkan not available: {:?}", e);
                None
            }
        }
    }
    
    /// Minimal valid SPIR-V header (may not pass full Vulkan validation)
    pub fn minimal_spirv_header() -> Vec<u8> {
        vec![
            0x03, 0x02, 0x23, 0x07, // Magic number
            0x00, 0x00, 0x01, 0x00, // Version 1.0
            0x00, 0x00, 0x00, 0x00, // Generator
            0x00, 0x00, 0x00, 0x00, // Bound
            0x00, 0x00, 0x00, 0x00, // Schema
        ]
    }
    
    /// Load a test shader file if it exists
    pub fn load_test_shader_bytes() -> Option<Vec<u8>> {
        // Try to load an actual valid shader for tests that need real SPIR-V
        std::fs::read("shaders/triangle.vert.spv").ok()
    }
}

// Unit Tests - Core Shader Loading
mod spirv_validation {
    use super::*;
    use super::helpers::*;
    
    #[test]
    fn test_valid_spirv_magic_number() {
        let Some(context) = create_test_context() else { return };
        
        // First test with real shader if available
        if let Some(real_spirv) = load_test_shader_bytes() {
            let result = ShaderModule::from_spirv_bytes(&context.device(), &real_spirv);
            assert!(result.is_ok(), "Failed to load valid shader: {:?}", result.err());
            return;
        }
        
        // Otherwise test with minimal header
        let valid_spirv = minimal_spirv_header();
        let result = ShaderModule::from_spirv_bytes(&context.device(), &valid_spirv);
        
        // Our validation should pass, but Vulkan's might be stricter
        match result {
            Ok(_) => println!("Minimal SPIR-V accepted by Vulkan"),
            Err(GammaVkError::ShaderCompilation { message }) => {
                // Should be Vulkan's error, not our magic number check
                assert!(
                    message.contains("Failed to create shader module"),
                    "Got unexpected error: {}",
                    message
                );
            }
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
    
    #[test]
    fn test_invalid_spirv_magic_number() {
        let Some(context) = create_test_context() else { return };
        
        // Wrong magic number
        let invalid_spirv = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // Invalid magic
            0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        
        let result = ShaderModule::from_spirv_bytes(&context.device(), &invalid_spirv);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            GammaVkError::ShaderCompilation { message } => {
                assert!(
                    message.contains("Invalid SPIR-V magic number"),
                    "Expected magic number error, got: {}",
                    message
                );
                assert!(
                    message.contains("expected 0x07230203"),
                    "Error should show expected value"
                );
                assert!(
                    message.contains("got 0xffffffff"),
                    "Error should show actual value"
                );
            }
            _ => panic!("Expected ShaderCompilation error"),
        }
    }
    
    #[test]
    fn test_spirv_bytecode_too_short() {
        let Some(context) = create_test_context() else { return };
        
        // Less than 4 bytes - can't even read magic number
        let too_short = vec![0x03, 0x02, 0x23];
        let result = ShaderModule::from_spirv_bytes(&context.device(), &too_short);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            GammaVkError::ShaderCompilation { message } => {
                assert!(
                    message.contains("multiple of 4 bytes"),
                    "Expected alignment error, got: {}",
                    message
                );
            }
            _ => panic!("Expected ShaderCompilation error"),
        }
    }
    
    #[test]
    fn test_spirv_bytecode_misaligned() {
        let Some(context) = create_test_context() else { return };
        
        // 5 bytes - not aligned to 4
        let misaligned = vec![0x03, 0x02, 0x23, 0x07, 0xFF];
        let result = ShaderModule::from_spirv_bytes(&context.device(), &misaligned);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            GammaVkError::ShaderCompilation { message } => {
                assert!(
                    message.contains("multiple of 4 bytes"),
                    "Expected alignment error, got: {}",
                    message
                );
            }
            _ => panic!("Expected ShaderCompilation error"),
        }
    }
    
    #[test]
    fn test_empty_spirv_bytecode() {
        let Some(context) = create_test_context() else { return };
        
        let empty: Vec<u8> = vec![];
        let result = ShaderModule::from_spirv_bytes(&context.device(), &empty);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            GammaVkError::ShaderCompilation { message } => {
                assert!(
                    message.contains("missing magic number"),
                    "Expected missing magic number error, got: {}",
                    message
                );
            }
            _ => panic!("Expected ShaderCompilation error"),
        }
    }
}

// Integration Tests - File Loading
mod file_loading {
    use super::*;
    use super::helpers::*;
    use std::fs;
    use std::path::Path;
    
    #[test]
    fn test_from_spirv_file_success() {
        let Some(context) = create_test_context() else { return };
        
        let test_shader_path = "shaders/triangle.vert.spv";
        
        // Check if file exists first
        if !Path::new(test_shader_path).exists() {
            println!("Test shader file not found at {} - skipping file test", test_shader_path);
            return;
        }
        
        let shader = ShaderModule::from_spirv_file(&context.device(), test_shader_path)
            .expect("Failed to load existing shader file");
        
        // Verify we can access the underlying module
        let _module = shader.vulkano_module();
    }
    
    #[test]
    fn test_from_spirv_file_missing() {
        let Some(context) = create_test_context() else { return };
        
        let nonexistent_path = "shaders/does_not_exist.spv";
        let result = ShaderModule::from_spirv_file(&context.device(), nonexistent_path);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            GammaVkError::ShaderCompilation { message } => {
                assert!(
                    message.contains("Failed to read shader file"),
                    "Expected file read error, got: {}",
                    message
                );
            }
            _ => panic!("Expected ShaderCompilation error"),
        }
    }
    
    #[test]
    fn test_shader_files_have_valid_spirv() {
        // Validate shader files if they exist
        let shader_paths = [
            ("shaders/triangle.vert.spv", "vertex"),
            ("shaders/triangle.frag.spv", "fragment"),
        ];
        
        for (path, shader_type) in &shader_paths {
            if let Ok(bytes) = fs::read(path) {
                assert!(
                    bytes.len() >= 4,
                    "{} shader file too small",
                    shader_type
                );
                assert!(
                    bytes.len() % 4 == 0,
                    "{} shader not aligned to 4 bytes",
                    shader_type
                );
                
                let magic = u32::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3]
                ]);
                assert_eq!(
                    magic, 0x07230203,
                    "{} shader has invalid SPIR-V magic number: 0x{:08x}",
                    shader_type, magic
                );
            } else {
                println!("{} shader not found at {} - skipping validation", shader_type, path);
            }
        }
    }
}

// Resource Management Tests
mod resource_management {
    use super::helpers::*;
    use gamma_vk::ShaderModule;
    use std::sync::Arc;
    
    #[test]
    fn test_shader_module_drop_cleanup() {
        let Some(context) = create_test_context() else { return };
        
        // Test that ShaderModule properly cleans up when dropped
        let device = context.device();
        
        // Use real shader if available, otherwise skip
        let spirv_bytes = match load_test_shader_bytes() {
            Some(bytes) => bytes,
            None => {
                println!("No test shader available - skipping drop cleanup test");
                return;
            }
        };
        
        // Create and drop shader in a scope
        {
            let _shader = ShaderModule::from_spirv_bytes(&device, &spirv_bytes)
                .expect("Failed to create shader for drop test");
        }
        
        // If we get here without crashing, RAII is working
        // In a real test, we might check GPU memory usage
    }
    
    #[test]
    fn test_multiple_shader_references() {
        let Some(context) = create_test_context() else { return };
        
        // Use real shader if available
        let spirv_bytes = match load_test_shader_bytes() {
            Some(bytes) => bytes,
            None => {
                println!("No test shader available - skipping reference counting test");
                return;
            }
        };
        
        // Test Arc reference counting with multiple references
        let shader = ShaderModule::from_spirv_bytes(&context.device(), &spirv_bytes)
            .expect("Failed to create shader for reference test");
        
        let module1 = shader.vulkano_module();
        let module2 = shader.vulkano_module();
        
        // Both should point to the same Arc
        assert!(Arc::ptr_eq(module1, module2));
    }
    
    #[test]
    fn test_shader_module_thread_safety() {
        let Some(_context) = create_test_context() else { return };
        
        // ShaderModule should be Send + Sync for thread safety
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ShaderModule>();
        
        // In practice, test with actual threading would be more complex
    }
}

// Error Handling Tests
mod error_handling {
    use super::*;
    use super::helpers::*;
    
    #[test]
    fn test_shader_compilation_error_type() {
        let Some(context) = create_test_context() else { return };
        
        // All shader errors should be ShaderCompilation variant
        let bad_spirv = vec![0xFF; 8];
        let result = ShaderModule::from_spirv_bytes(&context.device(), &bad_spirv);
        
        assert!(matches!(
            result,
            Err(GammaVkError::ShaderCompilation { .. })
        ));
    }
    
    #[test]
    fn test_error_messages_descriptive() {
        let Some(context) = create_test_context() else { return };
        
        // Test various error scenarios for message quality
        let test_cases = vec![
            (
                vec![0xFF; 4],
                "Invalid SPIR-V magic number",
                "Magic number error should be clear"
            ),
            (
                vec![0x03],
                "multiple of 4 bytes",
                "Alignment error should be clear"
            ),
            (
                vec![],
                "missing magic number",
                "Empty input error should be clear"
            ),
        ];
        
        for (input, expected_msg, test_desc) in test_cases {
            match ShaderModule::from_spirv_bytes(&context.device(), &input) {
                Err(GammaVkError::ShaderCompilation { message }) => {
                    assert!(
                        message.contains(expected_msg),
                        "{}: got '{}'",
                        test_desc,
                        message
                    );
                }
                _ => panic!("{}: expected error", test_desc),
            }
        }
    }
}

// Common shader loading functions
mod common_shaders {
    use super::helpers::*;
    
    #[test]
    fn test_load_triangle_vertex_shader() {
        let Some(context) = create_test_context() else { return };
        
        match gamma_vk::shader::common::load_triangle_vertex(&context.device()) {
            Ok(shader) => {
                let _module = shader.vulkano_module();
                println!("Successfully loaded common vertex shader");
            }
            Err(_) => {
                println!("Common vertex shader not available - this is expected if shaders/triangle.vert.spv doesn't exist");
            }
        }
    }
    
    #[test]
    fn test_load_triangle_fragment_shader() {
        let Some(context) = create_test_context() else { return };
        
        match gamma_vk::shader::common::load_triangle_fragment(&context.device()) {
            Ok(shader) => {
                let _module = shader.vulkano_module();
                println!("Successfully loaded common fragment shader");
            }
            Err(_) => {
                println!("Common fragment shader not available - this is expected if shaders/triangle.frag.spv doesn't exist");
            }
        }
    }
}

// Debug implementation tests
#[test]
fn test_shader_module_debug_format() {
    let Some(context) = helpers::create_test_context() else { return };
    
    // Use real shader if available
    let spirv_bytes = match helpers::load_test_shader_bytes() {
        Some(bytes) => bytes,
        None => {
            println!("No test shader available - skipping debug format test");
            return;
        }
    };
    
    let shader = ShaderModule::from_spirv_bytes(&context.device(), &spirv_bytes)
        .expect("Failed to create shader for debug test");
    
    let debug_str = format!("{:?}", shader);
    assert!(debug_str.contains("ShaderModule"));
    assert!(debug_str.contains("VulkanoShaderModule"));
    // Should not expose internal pointers or sensitive data
    assert!(!debug_str.contains("0x"));
}
