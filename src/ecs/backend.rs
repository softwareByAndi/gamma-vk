//! ECS backend trait for swappable implementations
//! 
//! This trait allows different storage strategies (sparse set, archetype, etc.)
//! to be used interchangeably while maintaining the same public API.

use crate::{Component, Entity, GammaVkError};
use std::any::TypeId;

/// Trait for ECS storage backends.
/// 
/// Implementations provide different performance characteristics:
/// - SparseSet: Fast component add/remove, slower iteration
/// - Archetype: Fast iteration, slower component changes
pub trait EcsBackend: Send + Sync + Default {
    /// Creates a new entity and returns its ID.
    fn create_entity(&mut self) -> Entity;
    
    /// Destroys an entity and all its components.
    fn destroy_entity(&mut self, entity: Entity) -> Result<(), GammaVkError>;
    
    /// Checks if an entity is alive.
    fn is_alive(&self, entity: Entity) -> bool;
    
    /// Adds a component to an entity.
    fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), GammaVkError>;
    
    /// Gets a component for an entity.
    fn get_component<C: Component>(&self, entity: Entity) -> Option<&C>;
    
    /// Gets a mutable component for an entity.
    fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C>;
    
    /// Removes a component from an entity.
    fn remove_component<C: Component>(&mut self, entity: Entity) -> Result<(), GammaVkError>;
    
    /// Queries for entities with a specific component type.
    /// Returns an iterator over (Entity, &Component) pairs.
    fn query_component<C: Component>(&self) -> Vec<(Entity, &C)>;
    
    /// Queries for entities with a specific component type (mutable).
    /// Returns an iterator over (Entity, &mut Component) pairs.
    fn query_component_mut<C: Component>(&mut self) -> Vec<(Entity, &mut C)>;
}