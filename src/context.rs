//! Vulkan context management for Gamma-VK
//!
//! This module provides the main VulkanContext struct that manages Vulkan instance
//! creation and provides a foundation for all graphics operations.

use std::sync::Arc;
use vulkano::{
    VulkanLibrary,
    device::{Device, DeviceCreateInfo, QueueCreateInfo, physical::PhysicalDevice},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
};

use crate::{GammaVkError, Result};

/// Main context for Vulkan operations
///
/// VulkanContext manages the Vulkan instance, device, and library, providing automatic
/// resource cleanup through RAII patterns. It handles MoltenVK compatibility
/// for macOS systems and provides graceful fallback options.
pub struct VulkanContext {
    /// The Vulkan instance
    pub instance: Arc<Instance>,
    /// The Vulkan library handle
    pub library: Arc<VulkanLibrary>,
    /// The logical device
    device: Arc<Device>,
    /// The selected physical device
    physical_device: Arc<PhysicalDevice>,
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

        // Select a physical device
        let physical_device = instance
            .enumerate_physical_devices()
            .map_err(|e| {
                GammaVkError::initialization(format!("Failed to enumerate physical devices: {}", e))
            })?
            .next()
            .ok_or_else(|| GammaVkError::initialization("No physical devices found"))?;

        // Find a graphics queue family
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, q)| {
                q.queue_flags
                    .intersects(vulkano::device::QueueFlags::GRAPHICS)
            })
            .ok_or_else(|| GammaVkError::initialization("No graphics queue family found"))?;

        // Create the logical device
        let (device, _queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: queue_family_index as u32,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .map_err(|e| GammaVkError::initialization(format!("Failed to create device: {}", e)))?;

        Ok(VulkanContext {
            instance,
            library,
            device,
            physical_device,
        })
    }

    /// Get information about enabled Vulkan layers
    pub fn enabled_layers(&self) -> &[String] {
        self.instance.enabled_layers()
    }

    /// Get information about enabled Vulkan extensions
    pub fn enabled_extensions(&self) -> &InstanceExtensions {
        self.instance.enabled_extensions()
    }

    /// Get a reference to the logical device
    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }

    /// Get a reference to the physical device
    pub fn physical_device(&self) -> Arc<PhysicalDevice> {
        self.physical_device.clone()
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
                match e {
                    GammaVkError::LibraryLoad(_) => {
                        panic!("Integration test: Library load failed (expected in CI)");
                    }
                    GammaVkError::InstanceCreation(_) => {
                        panic!("Integration test: Instance creation failed (expected in CI)");
                    }
                    _ => {
                        panic!("Integration test: Unexpected error: {}", e);
                    }
                }
            }
        }
    }
}
