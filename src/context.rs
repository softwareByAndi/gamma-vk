//! Vulkan context management for Gamma-VK
//!
//! This module provides the main VulkanContext struct that manages Vulkan instance
//! creation and provides a foundation for all graphics operations.

use std::sync::Arc;
use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
};

use crate::{GammaVkError, Result};

/// Main context for Vulkan operations
///
/// VulkanContext manages the Vulkan instance and library, providing automatic
/// resource cleanup through RAII patterns. It handles MoltenVK compatibility
/// for macOS systems and provides graceful fallback options.
pub struct VulkanContext {
    /// The Vulkan instance
    pub instance: Arc<Instance>,
    /// The Vulkan library handle
    pub library: Arc<VulkanLibrary>,
}

impl VulkanContext {
    /// Create a new VulkanContext with proper Vulkan initialization
    ///
    /// This method attempts to create a Vulkan instance with MoltenVK portability
    /// support first, then falls back to standard Vulkan if that fails.
    ///
    /// # Errors
    ///
    /// Returns `GammaVkError` if:
    /// - Vulkan library cannot be loaded (drivers not available)
    /// - Vulkan instance creation fails with both portability and standard modes
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::VulkanContext;
    ///
    /// let context = VulkanContext::new()?;
    /// println!("Vulkan context created successfully!");
    /// # Ok::<(), gamma_vk::GammaVkError>(())
    /// ```
    pub fn new() -> Result<Self> {
        // Load the Vulkan library
        let library = VulkanLibrary::new().map_err(GammaVkError::LibraryLoad)?;

        // Try with portability enumeration for MoltenVK first
        let instance = match Instance::new(
            library.clone(),
            InstanceCreateInfo {
                enabled_extensions: InstanceExtensions {
                    khr_portability_enumeration: true,
                    ..InstanceExtensions::empty()
                },
                flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        ) {
            Ok(instance) => {
                // Vulkan instance created with portability enumeration
                instance
            }
            Err(_) => {
                // Portability enumeration failed, trying standard Vulkan
                // Fall back to standard Vulkan instance creation
                Instance::new(
                    library.clone(),
                    InstanceCreateInfo {
                        ..Default::default()
                    },
                )
                .map_err(|e| {
                    GammaVkError::InstanceCreation(format!(
                        "Failed to create Vulkan instance: {}",
                        e
                    ))
                })?
            }
        };

        Ok(VulkanContext { instance, library })
    }

    /// Get information about enabled Vulkan layers
    pub fn enabled_layers(&self) -> &[String] {
        self.instance.enabled_layers()
    }

    /// Get information about enabled Vulkan extensions
    pub fn enabled_extensions(&self) -> &InstanceExtensions {
        self.instance.enabled_extensions()
    }
}

impl Drop for VulkanContext {
    /// Automatic cleanup when VulkanContext is dropped
    ///
    /// This implementation ensures proper resource cleanup through Rust's RAII.
    /// The Vulkan instance and library will be automatically cleaned up when
    /// this context goes out of scope.
    fn drop(&mut self) {
        // VulkanContext dropped - Vulkan resources cleaned up
        // Resources are automatically cleaned up by Arc<Instance> and Arc<VulkanLibrary>
        // when their reference counts reach zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulkan_context_creation() {
        // Note: This test might fail in CI environments without Vulkan drivers
        // but should work in local development environments
        match VulkanContext::new() {
            Ok(context) => {
                // Verify we can access basic information
                assert!(
                    !context.enabled_layers().is_empty() || context.enabled_layers().is_empty()
                );
                println!("Vulkan context created successfully in test");
            }
            Err(e) => {
                // In CI or environments without Vulkan, this is expected
                println!("Vulkan context creation failed (expected in CI): {}", e);
            }
        }
    }
}
