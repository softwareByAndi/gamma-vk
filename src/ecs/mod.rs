//! Entity Component System (ECS) for Gamma-VK
//! 
//! This module provides a flexible, performant ECS with swappable backend implementations.
//! 
//! # Example
//! ```
//! use gamma_vk::ecs::{World, Component};
//! 
//! #[derive(Debug, Clone)]
//! struct Position { x: f32, y: f32 }
//! impl Component for Position {}
//! 
//! let mut world = World::new()?;
//! 
//! let entity = world.spawn()
//!     .with(Position { x: 0.0, y: 0.0 })
//!     .build();
//! 
//! if let Some(pos) = world.get_mut::<Position>(entity) {
//!     pos.x += 1.0;
//! }
//! ```

mod entity;
mod component;
mod backend;
mod sparse_set;
mod sparse_set_backend;
mod world;

// Re-exports
pub use entity::Entity;
pub use component::Component;
pub use backend::EcsBackend;
pub use sparse_set_backend::SparseSetBackend;
pub use world::{World, EntityBuilder};

// For testing - temporary error type
pub use crate::error::GammaVkError;

// Module structure for organized development
pub mod error {
    // Placeholder for ECS errors that will be added to main GammaVkError
    use crate::Entity;
    
    #[derive(Debug)]
    pub enum GammaVkError {
        EntityNotFound(Entity),
        ComponentNotFound(Entity),
    }
    
    impl std::fmt::Display for GammaVkError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::EntityNotFound(e) => write!(f, "Entity not found: {:?}", e),
                Self::ComponentNotFound(e) => write!(f, "Component not found for entity: {:?}", e),
            }
        }
    }
    
    impl std::error::Error for GammaVkError {}
}