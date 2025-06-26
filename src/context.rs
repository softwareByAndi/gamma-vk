//! Vulkan context management for Gamma-VK
//!
//! This module provides the main VulkanContext struct that manages Vulkan instance
//! creation and provides a foundation for all graphics operations.

use std::sync::Arc;
use vulkano::{
    Version, VulkanLibrary,
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, physical::PhysicalDevice},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::StandardMemoryAllocator,
};

use crate::{GammaVkError, Result};

/// Builder for creating a VulkanContext with custom configuration
///
/// This builder pattern allows flexible configuration of the Vulkan instance
/// and device creation process.
///
/// # Examples
///
/// ```no_run
/// use gamma_vk::VulkanContext;
///
/// let context = VulkanContext::builder()
///     .application_name("My Game")
///     .application_version(1, 0, 0)
///     .engine_name("Gamma-VK")
///     .engine_version(0, 1, 0)
///     .enable_validation_layers()
///     .build()?;
/// # Ok::<(), gamma_vk::GammaVkError>(())
/// ```
#[derive(Debug, Clone)]
pub struct VulkanContextBuilder {
    application_name: Option<String>,
    application_version: Version,
    engine_name: Option<String>,
    engine_version: Version,
    enable_validation: bool,
    prefer_discrete_gpu: bool,
    required_extensions: Vec<String>,
}

impl Default for VulkanContextBuilder {
    fn default() -> Self {
        Self {
            application_name: None,
            application_version: Version::V1_0,
            engine_name: Some("Gamma-VK".to_string()),
            engine_version: Version::V1_0,
            enable_validation: cfg!(debug_assertions),
            prefer_discrete_gpu: true,
            required_extensions: Vec::new(),
        }
    }
}

impl VulkanContextBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name
    pub fn application_name(mut self, name: impl Into<String>) -> Self {
        self.application_name = Some(name.into());
        self
    }

    /// Set the application version
    pub fn application_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.application_version = Version {
            major,
            minor,
            patch,
        };
        self
    }

    /// Set the engine name
    pub fn engine_name(mut self, name: impl Into<String>) -> Self {
        self.engine_name = Some(name.into());
        self
    }

    /// Set the engine version
    pub fn engine_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.engine_version = Version {
            major,
            minor,
            patch,
        };
        self
    }

    /// Enable validation layers (enabled by default in debug builds)
    pub fn enable_validation_layers(mut self) -> Self {
        self.enable_validation = true;
        todo!("extensions are not yet implemented in VulkanContext");
        // self
    }

    /// Disable validation layers (useful for performance testing in debug builds)
    pub fn disable_validation_layers(mut self) -> Self {
        self.enable_validation = false;
        todo!("extensions are not yet implemented in VulkanContext");
        // self
    }

    /// Prefer discrete GPU over integrated (default: true)
    pub fn prefer_discrete_gpu(mut self, prefer: bool) -> Self {
        self.prefer_discrete_gpu = prefer;
        todo!("extensions are not yet implemented in VulkanContext");
        // self
    }

    /// Add a required instance extension
    pub fn required_extension(mut self, extension: impl Into<String>) -> Self {
        self.required_extensions.push(extension.into());
        todo!("extensions are not yet implemented in VulkanContext");
        // self
    }

    /// Build the VulkanContext with the configured settings
    pub fn build(self) -> Result<VulkanContext> {
        VulkanContext::new_with_config(self)
    }
}

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
    /// The graphics queue
    graphics_queue: Arc<Queue>,
    /// The graphics queue family index
    graphics_queue_family_index: u32,
    /// The memory allocator for GPU memory management
    memory_allocator: Arc<StandardMemoryAllocator>,
}

impl VulkanContext {
    /// Create a builder for configuring VulkanContext creation
    ///
    /// This is the recommended way to create a VulkanContext with custom settings.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::VulkanContext;
    ///
    /// let context = VulkanContext::builder()
    ///     .application_name("My App")
    ///     .enable_validation_layers()
    ///     .build()?;
    /// # Ok::<(), gamma_vk::GammaVkError>(())
    /// ```
    pub fn builder() -> VulkanContextBuilder {
        VulkanContextBuilder::default()
    }

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
        Self::builder().build()
    }

    /// Create a new VulkanContext with a specific configuration
    fn new_with_config(config: VulkanContextBuilder) -> Result<Self> {
        // Load the Vulkan library
        let library = VulkanLibrary::new().map_err(GammaVkError::LibraryLoad)?;

        // Build instance extensions
        let extensions = InstanceExtensions {
            khr_portability_enumeration: true,
            ..InstanceExtensions::empty()
        };

        // Note: Vulkano's extension system is compile-time based
        // Dynamic extension loading would require a different approach
        // For now, we just support the basic extensions needed

        // Try with portability enumeration for MoltenVK first
        let instance = match Instance::new(
            library.clone(),
            InstanceCreateInfo {
                application_name: config.application_name.clone(),
                application_version: config.application_version,
                engine_name: config.engine_name.clone(),
                engine_version: config.engine_version,
                enabled_extensions: extensions,
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
                        application_name: config.application_name,
                        application_version: config.application_version,
                        engine_name: config.engine_name,
                        engine_version: config.engine_version,
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
        let (device, mut queues) = Device::new(
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

        // Get the graphics queue
        let graphics_queue = queues
            .next()
            .ok_or_else(|| GammaVkError::initialization("Failed to get graphics queue"))?;

        // Create the memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        Ok(VulkanContext {
            instance,
            library,
            device,
            physical_device,
            graphics_queue,
            graphics_queue_family_index: queue_family_index as u32,
            memory_allocator,
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

    /// Get a reference to the graphics queue
    ///
    /// This queue supports graphics operations and is used for command submission.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::VulkanContext;
    ///
    /// let context = VulkanContext::new()?;
    /// let queue = context.graphics_queue();
    /// // Use queue for command submission
    /// # Ok::<(), gamma_vk::GammaVkError>(())
    /// ```
    pub fn graphics_queue(&self) -> Arc<Queue> {
        self.graphics_queue.clone()
    }

    /// Get the graphics queue family index
    ///
    /// This index identifies which queue family was selected for graphics operations.
    /// Useful when creating command pools or other resources that need a queue family.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::VulkanContext;
    ///
    /// let context = VulkanContext::new()?;
    /// let queue_family = context.graphics_queue_family_index();
    /// println!("Graphics queue family: {}", queue_family);
    /// # Ok::<(), gamma_vk::GammaVkError>(())
    /// ```
    pub fn graphics_queue_family_index(&self) -> u32 {
        self.graphics_queue_family_index
    }

    /// Get a reference to the memory allocator
    ///
    /// The memory allocator is used for all GPU memory allocations in the engine.
    /// This includes buffers, images, and other GPU resources.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gamma_vk::VulkanContext;
    ///
    /// let context = VulkanContext::new()?;
    /// let allocator = context.memory_allocator();
    /// // Use allocator for buffer/image creation
    /// # Ok::<(), gamma_vk::GammaVkError>(())
    /// ```
    pub fn memory_allocator(&self) -> Arc<StandardMemoryAllocator> {
        self.memory_allocator.clone()
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
