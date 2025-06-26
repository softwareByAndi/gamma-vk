//! Error handling for Gamma-VK
//!
//! This module provides comprehensive error types for the Gamma-VK graphics engine,
//! wrapping Vulkan errors and providing clear error information for users.

use thiserror::Error;

/// Main error type for Gamma-VK operations
///
/// This enum covers all possible errors that can occur during Gamma-VK operations,
/// providing context and source information for debugging and error recovery.
#[derive(Error, Debug)]
pub enum GammaVkError {
    /// Vulkan-specific errors from the underlying Vulkano library
    #[error("Vulkan error: {0}")]
    Vulkan(#[from] vulkano::VulkanError),

    /// Library loading errors when Vulkan drivers are not available
    #[error("Failed to load Vulkan library: {0}")]
    LibraryLoad(#[from] vulkano::LoadingError),

    /// Instance creation errors
    #[error("Failed to create Vulkan instance: {0}")]
    InstanceCreation(String),

    /// Generic initialization error
    #[error("Initialization failed: {message}")]
    Initialization { message: String },

    /// Buffer allocation and management errors
    #[error("Buffer operation failed: {message}")]
    BufferCreation { message: String },

    /// Shader compilation and loading errors
    #[error("Shader compilation failed: {message}")]
    ShaderCompilation { message: String },
}

impl GammaVkError {
    /// Create a new initialization error with a custom message
    pub fn initialization<S: Into<String>>(message: S) -> Self {
        Self::Initialization {
            message: message.into(),
        }
    }

    /// Create a new buffer creation error with a custom message
    pub fn buffer_creation<S: Into<String>>(message: S) -> Self {
        Self::BufferCreation {
            message: message.into(),
        }
    }

    /// Create a new shader compilation error with a custom message
    pub fn shader_compilation<S: Into<String>>(message: S) -> Self {
        Self::ShaderCompilation {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_error_creation() {
        let error = GammaVkError::initialization("test error");
        match error {
            GammaVkError::Initialization { message } => {
                assert_eq!(message, "test error");
            }
            _ => panic!("Expected initialization error"),
        }
    }

    #[test]
    fn test_error_display() {
        let error = GammaVkError::initialization("display test");
        let error_string = format!("{}", error);
        assert!(error_string.contains("Initialization failed"));
        assert!(error_string.contains("display test"));
    }

    #[test]
    fn test_shader_compilation_error_creation() {
        let error = GammaVkError::shader_compilation("invalid SPIR-V bytecode");
        match error {
            GammaVkError::ShaderCompilation { message } => {
                assert_eq!(message, "invalid SPIR-V bytecode");
            }
            _ => panic!("Expected shader compilation error"),
        }
    }
}
