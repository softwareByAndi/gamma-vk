//! World - the main entry point for the ECS
//! 
//! World manages entities, components, and systems. It provides a type-safe
//! API over the underlying ECS backend.

use crate::{backend::EcsBackend, Component, Entity, GammaVkError, SparseSetBackend};

/// The main ECS world that manages entities and components.
/// 
/// World is generic over the backend implementation, allowing different
/// storage strategies to be used.
pub struct World<B: EcsBackend = SparseSetBackend> {
    backend: B,
}

impl<B: EcsBackend> World<B> {
    /// Creates a new empty world.
    pub fn new() -> Result<Self, GammaVkError> {
        Ok(Self {
            backend: B::default(),
        })
    }
    
    /// Creates a new entity using the builder pattern.
    pub fn spawn(&mut self) -> EntityBuilder<B> {
        let entity = self.backend.create_entity();
        EntityBuilder {
            world: self,
            entity,
        }
    }
    
    /// Destroys an entity and all its components.
    pub fn destroy(&mut self, entity: Entity) -> Result<(), GammaVkError> {
        self.backend.destroy_entity(entity)
    }
    
    /// Checks if an entity is alive.
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.backend.is_alive(entity)
    }
    
    /// Gets a component for an entity.
    pub fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.backend.get_component::<C>(entity)
    }
    
    /// Gets a mutable component for an entity.
    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        self.backend.get_component_mut::<C>(entity)
    }
    
    /// Adds a component to an entity.
    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), GammaVkError> {
        self.backend.add_component(entity, component)
    }
    
    /// Removes a component from an entity.
    pub fn remove<C: Component>(&mut self, entity: Entity) -> Result<(), GammaVkError> {
        self.backend.remove_component::<C>(entity)
    }
    
    /// Queries for all entities with a specific component.
    pub fn query<C: Component>(&self) -> impl Iterator<Item = (Entity, &C)> {
        self.backend.query_component::<C>().into_iter()
    }
    
    /// Queries for all entities with a specific component (mutable).
    pub fn query_mut<C: Component>(&mut self) -> impl Iterator<Item = (Entity, &mut C)> {
        self.backend.query_component_mut::<C>().into_iter()
    }
}

/// Builder for creating entities with components.
pub struct EntityBuilder<'a, B: EcsBackend> {
    world: &'a mut World<B>,
    entity: Entity,
}

impl<'a, B: EcsBackend> EntityBuilder<'a, B> {
    /// Adds a component to the entity being built.
    pub fn with<C: Component>(self, component: C) -> Self {
        // Ignore errors during building - entity is already created
        let _ = self.world.add_component(self.entity, component);
        self
    }
    
    /// Finishes building and returns the entity.
    pub fn build(self) -> Entity {
        self.entity
    }
}

// Query API for multiple components - simplified version for Phase 1
impl<B: EcsBackend> World<B> {
    /// Queries for entities with two components.
    /// 
    /// This is a simplified implementation for Phase 1.
    /// Phase 3 will add a more sophisticated query system.
    pub fn query2<A: Component, B: Component>(&self) -> Vec<(Entity, (&A, &B))> {
        let mut results = Vec::new();
        
        // Get all entities with component A
        for (entity, a) in self.query::<A>() {
            // Check if they also have component B
            if let Some(b) = self.get::<B>(entity) {
                results.push((entity, (a, b)));
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity { dx: f32, dy: f32 }
    impl Component for Velocity {}

    #[test]
    fn test_world_creation() {
        let world = World::<SparseSetBackend>::new();
        assert!(world.is_ok());
    }

    #[test]
    fn test_entity_builder() {
        let mut world = World::<SparseSetBackend>::new().unwrap();
        
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { dx: 0.5, dy: -0.5 })
            .build();
        
        assert!(world.is_alive(entity));
        assert_eq!(world.get::<Position>(entity), Some(&Position { x: 1.0, y: 2.0 }));
        assert_eq!(world.get::<Velocity>(entity), Some(&Velocity { dx: 0.5, dy: -0.5 }));
    }

    #[test]
    fn test_query_single_component() {
        let mut world = World::<SparseSetBackend>::new().unwrap();
        
        let e1 = world.spawn().with(Position { x: 1.0, y: 1.0 }).build();
        let _e2 = world.spawn().with(Velocity { dx: 1.0, dy: 0.0 }).build();
        let e3 = world.spawn().with(Position { x: 3.0, y: 3.0 }).build();
        
        let positions: Vec<_> = world.query::<Position>()
            .map(|(e, _)| e)
            .collect();
        
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&e1));
        assert!(positions.contains(&e3));
    }

    #[test]
    fn test_query_multiple_components() {
        let mut world = World::<SparseSetBackend>::new().unwrap();
        
        let e1 = world.spawn()
            .with(Position { x: 1.0, y: 1.0 })
            .with(Velocity { dx: 0.5, dy: 0.5 })
            .build();
            
        let _e2 = world.spawn()
            .with(Position { x: 2.0, y: 2.0 })
            .build();
        
        let results = world.query2::<Position, Velocity>();
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, e1);
    }
}