//! Gamma-VK: A safe, performant Vulkan graphics engine built in Rust
//!
//! This library provides a high-level interface for Vulkan graphics programming
//! with automatic resource management through RAII patterns.

pub mod buffer;
pub mod context;
pub mod error;

// Re-export main types for easy library usage
pub use buffer::{Buffer, IndexBuffer, UniformBuffer, VertexBuffer};
pub use context::VulkanContext;
pub use error::GammaVkError;

/// Result type alias for convenient error handling throughout the library
pub type Result<T> = std::result::Result<T, GammaVkError>;
