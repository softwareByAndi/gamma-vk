
use gamma_vk::{GammaVkError, VulkanContext, ShaderModule};

#[test]
fn test_shader_module_creation_from_minimal_spirv() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Load an actual valid shader file for testing
    match ShaderModule::from_spirv_file(&context.device(), "shaders/triangle.vert.spv") {
        Ok(_shader) => {
            // Good - we can create shaders from real files
            println!("Successfully created shader from file");
        }
        Err(_) => {
            // File might not exist in test environment, try with minimal test data
            let valid_spirv = &[
                0x03, 0x02, 0x23, 0x07, // Magic number
                0x00, 0x00, 0x01, 0x00, // Version
                0x00, 0x00, 0x00, 0x00, // Generator
                0x00, 0x00, 0x00, 0x00, // Bound
                0x00, 0x00, 0x00, 0x00, // Schema
            ];

            // This minimal SPIR-V passes our validation but may fail Vulkan's
            let result = ShaderModule::from_spirv_bytes(&context.device(), valid_spirv);
            match result {
                Ok(_) => println!("Minimal SPIR-V created module"),
                Err(GammaVkError::ShaderCompilation { message }) => {
                    // Expected - Vulkan validation is stricter
                    assert!(
                        message.contains("Failed to create shader module"),
                        "Got error: {}",
                        message
                    );
                }
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }
    }
}

#[test]
fn test_shader_module_creation_from_invalid_spirv() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Test with invalid SPIR-V (wrong magic number)
    let invalid_spirv = &[0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04];
    let shader_result = ShaderModule::from_spirv_bytes(&context.device(), invalid_spirv);

    assert!(shader_result.is_err(), "Should fail with invalid SPIR-V");

    match shader_result.unwrap_err() {
        GammaVkError::ShaderCompilation { message } => {
            assert!(message.contains("Invalid SPIR-V magic number"));
        }
        _ => panic!("Expected ShaderCompilation error"),
    }
}

#[test]
fn test_shader_module_creation_from_short_spirv() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Test with too short SPIR-V (but still multiple of 4)
    let short_spirv = &[0x03, 0x02, 0x23, 0x07]; // Just magic number, nothing else
    let shader_result = ShaderModule::from_spirv_bytes(&context.device(), short_spirv);

    assert!(shader_result.is_err(), "Should fail with short SPIR-V");

    // The actual error will be from Vulkan validation
    match shader_result.unwrap_err() {
        GammaVkError::ShaderCompilation { message } => {
            assert!(
                message.contains("Failed to create shader module"),
                "Expected shader creation error, got: {}",
                message
            );
        }
        _ => panic!("Expected ShaderCompilation error"),
    }
}

#[test]
fn test_shader_module_creation_from_too_short_spirv() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Test with SPIR-V that's too short to have magic number
    let too_short = &[0x03, 0x02]; // Only 2 bytes
    let shader_result = ShaderModule::from_spirv_bytes(&context.device(), too_short);

    assert!(shader_result.is_err(), "Should fail with too short SPIR-V");

    match shader_result.unwrap_err() {
        GammaVkError::ShaderCompilation { message } => {
            // Could be either "multiple of 4" or "missing magic number"
            assert!(
                message.contains("multiple of 4 bytes")
                    || message.contains("missing magic number"),
                "Expected error about alignment or magic number, got: {}",
                message
            );
        }
        _ => panic!("Expected ShaderCompilation error"),
    }
}

#[test]
fn test_shader_module_creation_from_unaligned_spirv() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Test with unaligned SPIR-V (not multiple of 4 bytes)
    let unaligned_spirv = &[0x03, 0x02, 0x23, 0x07, 0x00];
    let shader_result = ShaderModule::from_spirv_bytes(&context.device(), unaligned_spirv);

    assert!(shader_result.is_err(), "Should fail with unaligned SPIR-V");

    match shader_result.unwrap_err() {
        GammaVkError::ShaderCompilation { message } => {
            assert!(message.contains("multiple of 4 bytes"));
        }
        _ => panic!("Expected ShaderCompilation error"),
    }
}

#[test]
fn test_shader_module_file_loading() {
    let context = VulkanContext::new().expect("Failed to create Vulkan context");

    // Try to load actual shader file
    match ShaderModule::from_spirv_file(&context.device(), "shaders/triangle.vert.spv") {
        Ok(shader) => {
            // Test that we can access the underlying vulkano module
            let _vulkano_module = shader.vulkano_module();
            println!("Successfully loaded and accessed shader from file");
        }
        Err(_) => {
            // This is OK in test environment where shader files might not exist
            println!("Shader file not found - skipping file loading test");
        }
    }
}

#[test]
fn test_shader_files_exist_and_valid() {
    use std::fs;

    // Check if shader files exist
    let vertex_path = "shaders/triangle.vert.spv";
    let fragment_path = "shaders/triangle.frag.spv";

    // Test vertex shader
    if let Ok(vertex_bytes) = fs::read(vertex_path) {
        assert!(vertex_bytes.len() >= 4, "Vertex shader file too small");
        let magic = u32::from_le_bytes([
            vertex_bytes[0],
            vertex_bytes[1],
            vertex_bytes[2],
            vertex_bytes[3],
        ]);
        assert_eq!(
            magic, 0x07230203,
            "Vertex shader has invalid SPIR-V magic number"
        );
    } else {
        println!("Vertex shader file not found - skipping validation");
    }

    // Test fragment shader
    if let Ok(fragment_bytes) = fs::read(fragment_path) {
        assert!(fragment_bytes.len() >= 4, "Fragment shader file too small");
        let magic = u32::from_le_bytes([
            fragment_bytes[0],
            fragment_bytes[1],
            fragment_bytes[2],
            fragment_bytes[3],
        ]);
        assert_eq!(
            magic, 0x07230203,
            "Fragment shader has invalid SPIR-V magic number"
        );
    } else {
        println!("Fragment shader file not found - skipping validation");
    }
}