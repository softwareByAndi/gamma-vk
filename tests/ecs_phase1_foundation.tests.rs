//! Phase 1: Foundation Tests for ECS
//! 
//! These tests define the expected behavior of the core ECS functionality.
//! Following TDD principles, these tests are written before implementation.

use gamma_vk::ecs::{World, Entity, Component, EcsBackend, SparseSetBackend};
use gamma_vk::error::GammaVkError;

#[cfg(test)]
mod entity_tests {
    use super::*;

    #[test]
    fn test_entity_creation_returns_unique_id() {
        // Given: A new ECS world
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        // When: Creating multiple entities
        let entity1 = world.spawn().build();
        let entity2 = world.spawn().build();
        let entity3 = world.spawn().build();
        
        // Then: Each entity has a unique ID
        assert_ne!(entity1.id(), entity2.id());
        assert_ne!(entity2.id(), entity3.id());
        assert_ne!(entity1.id(), entity3.id());
    }

    #[test]
    fn test_entity_destruction_removes_all_components() {
        // Given: An entity with multiple components
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { dx: 0.5, dy: -0.5 })
            .build();
        
        // When: Destroying the entity
        world.destroy(entity).expect("Failed to destroy entity");
        
        // Then: Components are no longer accessible
        assert!(world.get::<Position>(entity).is_none());
        assert!(world.get::<Velocity>(entity).is_none());
    }

    #[test]
    fn test_entity_id_not_reused_immediately() {
        // Given: A world with a destroyed entity
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity1 = world.spawn().build();
        let id1 = entity1.id();
        world.destroy(entity1).expect("Failed to destroy entity");
        
        // When: Creating new entities
        let entity2 = world.spawn().build();
        let entity3 = world.spawn().build();
        
        // Then: The old ID is not immediately reused
        assert_ne!(id1, entity2.id());
        assert_ne!(id1, entity3.id());
    }

    #[test]
    fn test_destroying_nonexistent_entity_returns_error() {
        // Given: A world and a fake entity ID
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let fake_entity = Entity::from_raw_parts(9999, 0);
        
        // When: Attempting to destroy non-existent entity
        let result = world.destroy(fake_entity);
        
        // Then: An appropriate error is returned
        assert!(matches!(result, Err(GammaVkError::EntityNotFound(_))));
    }
}

#[cfg(test)]
mod component_tests {
    use super::*;

    // Test components
    #[derive(Debug, Clone, PartialEq)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity { dx: f32, dy: f32 }
    impl Component for Velocity {}

    #[derive(Debug, Clone)]
    struct Name(String);
    impl Component for Name {}

    #[test]
    fn test_component_storage_type_safe_access() {
        // Given: An entity with a Position component
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn()
            .with(Position { x: 10.0, y: 20.0 })
            .build();
        
        // When: Accessing the component
        let position = world.get::<Position>(entity);
        
        // Then: The correct component is returned
        assert!(position.is_some());
        assert_eq!(position.unwrap().x, 10.0);
        assert_eq!(position.unwrap().y, 20.0);
    }

    #[test]
    fn test_component_removal_cleans_memory() {
        // Given: An entity with components
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { dx: 0.0, dy: 0.0 })
            .build();
        
        // When: Removing a component
        world.remove::<Position>(entity).expect("Failed to remove component");
        
        // Then: Only the removed component is gone
        assert!(world.get::<Position>(entity).is_none());
        assert!(world.get::<Velocity>(entity).is_some());
    }

    #[test]
    fn test_missing_component_returns_none() {
        // Given: An entity without a Velocity component
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn()
            .with(Position { x: 0.0, y: 0.0 })
            .build();
        
        // When: Trying to access missing component
        let velocity = world.get::<Velocity>(entity);
        
        // Then: None is returned (no panic)
        assert!(velocity.is_none());
    }

    #[test]
    fn test_mutable_component_access() {
        // Given: An entity with a Position
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn()
            .with(Position { x: 0.0, y: 0.0 })
            .build();
        
        // When: Modifying the component
        if let Some(pos) = world.get_mut::<Position>(entity) {
            pos.x = 5.0;
            pos.y = 10.0;
        }
        
        // Then: Changes are persisted
        let position = world.get::<Position>(entity).unwrap();
        assert_eq!(position.x, 5.0);
        assert_eq!(position.y, 10.0);
    }

    #[test]
    fn test_component_type_safety() {
        // This test verifies compile-time type safety
        // The following should not compile:
        // let mut world = World::<SparseSetBackend>::new().unwrap();
        // let entity = world.spawn().build();
        // world.add_component(entity, "not a component"); // Should fail
        
        // Instead, only proper components can be added:
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        let entity = world.spawn().build();
        
        // These compile because Position implements Component
        world.add_component(entity, Position { x: 0.0, y: 0.0 })
            .expect("Failed to add component");
    }
}

#[cfg(test)]
mod query_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_query_entities_with_single_component() {
        // Given: Multiple entities, some with Position
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        let entity1 = world.spawn()
            .with(Position { x: 1.0, y: 1.0 })
            .build();
            
        let _entity2 = world.spawn()
            .with(Velocity { dx: 1.0, dy: 0.0 })
            .build();
            
        let entity3 = world.spawn()
            .with(Position { x: 3.0, y: 3.0 })
            .build();
        
        // When: Querying for entities with Position
        let mut found_entities = HashSet::new();
        for (entity, _pos) in world.query::<&Position>() {
            found_entities.insert(entity.id());
        }
        
        // Then: Only entities with Position are returned
        assert_eq!(found_entities.len(), 2);
        assert!(found_entities.contains(&entity1.id()));
        assert!(found_entities.contains(&entity3.id()));
    }

    #[test]
    fn test_query_entities_with_multiple_components() {
        // Given: Entities with various component combinations
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        let entity1 = world.spawn()
            .with(Position { x: 1.0, y: 1.0 })
            .with(Velocity { dx: 0.5, dy: 0.5 })
            .build();
            
        let _entity2 = world.spawn()
            .with(Position { x: 2.0, y: 2.0 })
            .build();
            
        let _entity3 = world.spawn()
            .with(Velocity { dx: 1.0, dy: 0.0 })
            .build();
        
        // When: Querying for entities with both Position AND Velocity
        let mut count = 0;
        for (entity, (_pos, _vel)) in world.query::<(&Position, &Velocity)>() {
            assert_eq!(entity.id(), entity1.id());
            count += 1;
        }
        
        // Then: Only entities with both components are returned
        assert_eq!(count, 1);
    }

    #[test]
    fn test_mutable_query_allows_modification() {
        // Given: Entities with Position components
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        let entity1 = world.spawn()
            .with(Position { x: 0.0, y: 0.0 })
            .build();
            
        let entity2 = world.spawn()
            .with(Position { x: 0.0, y: 0.0 })
            .build();
        
        // When: Modifying through a mutable query
        for (_entity, pos) in world.query_mut::<&mut Position>() {
            pos.x += 1.0;
            pos.y += 1.0;
        }
        
        // Then: All queried components are modified
        assert_eq!(world.get::<Position>(entity1).unwrap().x, 1.0);
        assert_eq!(world.get::<Position>(entity2).unwrap().x, 1.0);
    }

    #[test]
    fn test_query_with_entity_id() {
        // Given: Multiple entities
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        let entities: Vec<_> = (0..5).map(|i| {
            world.spawn()
                .with(Position { x: i as f32, y: 0.0 })
                .build()
        }).collect();
        
        // When: Querying with entity access
        let mut found_ids = Vec::new();
        for (entity, pos) in world.query::<&Position>() {
            found_ids.push(entity.id());
            // Verify we can use the entity ID to access other data
            assert_eq!(pos.x as usize, entities.iter().position(|&e| e.id() == entity.id()).unwrap());
        }
        
        // Then: All entities are accessible
        assert_eq!(found_ids.len(), 5);
    }
}

#[cfg(test)]
mod builder_pattern_tests {
    use super::*;

    #[test]
    fn test_entity_builder_fluent_api() {
        // Given: A world
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        // When: Using the builder pattern
        let entity = world.spawn()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { dx: 0.5, dy: -0.5 })
            .with(Name("Player".to_string()))
            .build();
        
        // Then: All components are properly attached
        assert!(world.get::<Position>(entity).is_some());
        assert!(world.get::<Velocity>(entity).is_some());
        assert!(world.get::<Name>(entity).is_some());
    }

    #[test]
    fn test_empty_entity_creation() {
        // Given: A world
        let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
        
        // When: Creating an entity without components
        let entity = world.spawn().build();
        
        // Then: Entity exists but has no components
        assert!(world.is_alive(entity));
        assert!(world.get::<Position>(entity).is_none());
    }
}

#[cfg(test)]
mod raii_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct ResourceTracker {
        counter: Arc<Mutex<i32>>,
    }

    impl ResourceTracker {
        fn new(counter: Arc<Mutex<i32>>) -> Self {
            *counter.lock().unwrap() += 1;
            Self { counter }
        }
    }

    impl Drop for ResourceTracker {
        fn drop(&mut self) {
            *self.counter.lock().unwrap() -= 1;
        }
    }

    impl Component for ResourceTracker {}

    #[test]
    fn test_component_drop_on_entity_destruction() {
        // Given: A counter to track resource lifecycle
        let counter = Arc::new(Mutex::new(0));
        
        {
            let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
            
            // When: Creating entity with tracked component
            let entity = world.spawn()
                .with(ResourceTracker::new(counter.clone()))
                .build();
            
            assert_eq!(*counter.lock().unwrap(), 1);
            
            // And: Destroying the entity
            world.destroy(entity).expect("Failed to destroy entity");
        }
        
        // Then: Component is dropped (RAII cleanup)
        assert_eq!(*counter.lock().unwrap(), 0);
    }

    #[test]
    fn test_component_drop_on_world_drop() {
        // Given: A counter to track resource lifecycle
        let counter = Arc::new(Mutex::new(0));
        
        {
            let mut world = World::<SparseSetBackend>::new().expect("Failed to create world");
            
            // When: Creating entities with tracked components
            for _ in 0..5 {
                world.spawn()
                    .with(ResourceTracker::new(counter.clone()))
                    .build();
            }
            
            assert_eq!(*counter.lock().unwrap(), 5);
            // World drops here
        }
        
        // Then: All components are dropped
        assert_eq!(*counter.lock().unwrap(), 0);
    }
}