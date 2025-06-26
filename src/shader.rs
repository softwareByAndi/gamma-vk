//! Shader management for Gamma-VK
//!
//! This module provides RAII-managed shader types with automatic resource cleanup
//! and type-safe shader loading from SPIR-V bytecode.

use std::{fs, path::Path, sync::Arc};
use vulkano::{
    device::Device,
    shader::{ShaderModule as VulkanoShaderModule, ShaderModuleCreateInfo},
};

use crate::{GammaVkError, Result};

/// A managed shader module wrapper providing RAII resource management
///
/// ShaderModule wraps a Vulkano shader module and provides automatic cleanup through
/// Rust's ownership system. It ensures proper resource lifecycle management
/// and prevents memory leaks.
pub struct ShaderModule {
    /// The underlying Vulkano shader module
    module: Arc<VulkanoShaderModule>,
}

impl ShaderModule {
    /// Create a new shader module from a compiled SPIR-V file
    ///
    /// # Arguments
    ///
    /// * `device` - The Vulkan device to create the shader module on
    /// * `path` - Path to the compiled .spv file
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created shader module or an error if creation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::shader::ShaderModule;
    /// use gamma_vk::context::VulkanContext;
    ///
    /// # fn example() -> gamma_vk::Result<()> {
    /// let context = VulkanContext::new()?;
    /// let shader = ShaderModule::from_spirv_file(&context.device(), "shaders/triangle.vert.spv")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The SPIR-V bytecode is invalid
    /// - Vulkan shader module creation fails
    pub fn from_spirv_file(device: &Arc<Device>, path: impl AsRef<Path>) -> Result<Self> {
        let spirv_bytes = fs::read(path.as_ref()).map_err(|e| {
            GammaVkError::shader_compilation(format!("Failed to read shader file: {}", e))
        })?;

        Self::from_spirv_bytes(device, &spirv_bytes)
    }

    /// Create a new shader module from SPIR-V bytecode
    ///
    /// # Arguments
    ///
    /// * `device` - The Vulkan device to create the shader module on
    /// * `spirv_bytes` - The SPIR-V bytecode as a byte slice
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created shader module or an error if creation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamma_vk::shader::ShaderModule;
    /// use gamma_vk::context::VulkanContext;
    ///
    /// # fn example() -> gamma_vk::Result<()> {
    /// let context = VulkanContext::new()?;
    /// let spirv_data = &[0x03, 0x02, 0x23, 0x07]; // Valid SPIR-V magic number
    /// let shader = ShaderModule::from_spirv_bytes(&context.device(), spirv_data)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The SPIR-V bytecode is invalid or corrupted
    /// * The shader module creation fails on the device
    /// * The device does not support the shader features used
    pub fn from_spirv_bytes(device: &Arc<Device>, spirv_bytes: &[u8]) -> Result<Self> {
        // Convert bytes to u32 words for SPIR-V validation
        if spirv_bytes.len() % 4 != 0 {
            return Err(GammaVkError::shader_compilation(
                "SPIR-V bytecode length must be a multiple of 4 bytes",
            ));
        }

        // Validate SPIR-V magic number
        if spirv_bytes.len() < 4 {
            return Err(GammaVkError::shader_compilation(
                "SPIR-V bytecode too short - missing magic number",
            ));
        }

        let magic_bytes = &spirv_bytes[0..4];
        let magic = u32::from_le_bytes([
            magic_bytes[0],
            magic_bytes[1],
            magic_bytes[2],
            magic_bytes[3],
        ]);

        if magic != 0x07230203 {
            return Err(GammaVkError::shader_compilation(format!(
                "Invalid SPIR-V magic number: expected 0x07230203, got 0x{:08x}",
                magic
            )));
        }

        // Convert to u32 words
        let spirv_words: Vec<u32> = spirv_bytes
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();

        // Create the shader module
        // Safety: We've validated the SPIR-V magic number and word alignment above
        let create_info = ShaderModuleCreateInfo::new(&spirv_words);
        let module =
            unsafe { VulkanoShaderModule::new(device.clone(), create_info) }.map_err(|e| {
                GammaVkError::shader_compilation(format!("Failed to create shader module: {}", e))
            })?;

        Ok(Self { module })
    }

    /// Get a reference to the underlying Vulkano shader module
    ///
    /// This provides an escape hatch for advanced users who need direct access
    /// to the underlying Vulkano shader module for features not yet wrapped
    /// by Gamma-VK.
    pub fn vulkano_module(&self) -> &Arc<VulkanoShaderModule> {
        &self.module
    }
}

impl std::fmt::Debug for ShaderModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShaderModule")
            .field("module", &"VulkanoShaderModule")
            .finish()
    }
}

/// Convenience functions for loading common shaders
pub mod common {
    use super::*;

    /// Load the basic triangle vertex shader
    pub fn load_triangle_vertex(device: &Arc<Device>) -> Result<ShaderModule> {
        ShaderModule::from_spirv_file(device, "shaders/triangle.vert.spv")
    }

    /// Load the basic triangle fragment shader
    pub fn load_triangle_fragment(device: &Arc<Device>) -> Result<ShaderModule> {
        ShaderModule::from_spirv_file(device, "shaders/triangle.frag.spv")
    }
}
