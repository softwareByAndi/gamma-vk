//! Sparse set backend implementation for the ECS
//! 
//! This backend uses sparse sets for component storage, providing:
//! - O(1) component insertion and removal
//! - O(1) component access
//! - Less cache-friendly iteration compared to archetype storage

use crate::{backend::EcsBackend, Component, ComponentStorage, Entity, GammaVkError, sparse_set::SparseSet};
use std::any::TypeId;
use std::collections::HashMap;

/// Entity metadata for tracking alive/dead state and generation.
#[derive(Debug, Clone)]
struct EntityMeta {
    generation: u32,
    alive: bool,
}

/// Sparse set backend for ECS storage.
pub struct SparseSetBackend {
    /// Entity metadata storage
    entities: Vec<EntityMeta>,
    
    /// Free list for entity ID reuse
    free_list: Vec<u32>,
    
    /// Component storages by type
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl Default for SparseSetBackend {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            free_list: Vec::new(),
            storages: HashMap::new(),
        }
    }
}

impl SparseSetBackend {
    /// Gets or creates a storage for a component type.
    fn get_or_create_storage<C: Component>(&mut self) -> &mut SparseSet<C> {
        let type_id = TypeId::of::<C>();
        
        self.storages
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<C>::new()))
            .as_any_mut()
            .downcast_mut::<SparseSet<C>>()
            .expect("Storage type mismatch")
    }
    
    /// Gets a storage for a component type if it exists.
    fn get_storage<C: Component>(&self) -> Option<&SparseSet<C>> {
        let type_id = TypeId::of::<C>();
        
        self.storages
            .get(&type_id)
            .and_then(|storage| storage.as_any().downcast_ref::<SparseSet<C>>())
    }
    
    /// Gets a mutable storage for a component type if it exists.
    fn get_storage_mut<C: Component>(&mut self) -> Option<&mut SparseSet<C>> {
        let type_id = TypeId::of::<C>();
        
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.as_any_mut().downcast_mut::<SparseSet<C>>())
    }
}

impl EcsBackend for SparseSetBackend {
    fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.free_list.pop() {
            // Reuse ID with incremented generation
            let meta = &mut self.entities[id as usize];
            meta.generation = meta.generation.wrapping_add(1);
            meta.alive = true;
            Entity::from_raw_parts(id, meta.generation)
        } else {
            // Allocate new ID
            let id = self.entities.len() as u32;
            self.entities.push(EntityMeta {
                generation: 0,
                alive: true,
            });
            Entity::from_raw_parts(id, 0)
        }
    }
    
    fn destroy_entity(&mut self, entity: Entity) -> Result<(), GammaVkError> {
        let index = entity.index() as usize;
        
        // Check entity exists and generation matches
        if index >= self.entities.len() {
            return Err(GammaVkError::EntityNotFound(entity));
        }
        
        let meta = &mut self.entities[index];
        if !meta.alive || meta.generation != entity.generation() {
            return Err(GammaVkError::EntityNotFound(entity));
        }
        
        // Mark as dead
        meta.alive = false;
        
        // Remove all components for this entity
        for storage in self.storages.values_mut() {
            storage.clear_for_entity(entity);
        }
        
        // Add to free list for reuse
        self.free_list.push(entity.index());
        
        Ok(())
    }
    
    fn is_alive(&self, entity: Entity) -> bool {
        let index = entity.index() as usize;
        
        self.entities
            .get(index)
            .map(|meta| meta.alive && meta.generation == entity.generation())
            .unwrap_or(false)
    }
    
    fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), GammaVkError> {
        if !self.is_alive(entity) {
            return Err(GammaVkError::EntityNotFound(entity));
        }
        
        let storage = self.get_or_create_storage::<C>();
        storage.insert(entity, component);
        Ok(())
    }
    
    fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
        if !self.is_alive(entity) {
            return None;
        }
        
        self.get_storage::<C>()
            .and_then(|storage| storage.get(entity))
    }
    
    fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        if !self.is_alive(entity) {
            return None;
        }
        
        self.get_storage_mut::<C>()
            .and_then(|storage| storage.get_mut(entity))
    }
    
    fn remove_component<C: Component>(&mut self, entity: Entity) -> Result<(), GammaVkError> {
        if !self.is_alive(entity) {
            return Err(GammaVkError::EntityNotFound(entity));
        }
        
        if let Some(storage) = self.get_storage_mut::<C>() {
            storage.remove(entity);
        }
        
        Ok(())
    }
    
    fn query_component<C: Component>(&self) -> Vec<(Entity, &C)> {
        self.get_storage::<C>()
            .map(|storage| storage.iter().collect())
            .unwrap_or_default()
    }
    
    fn query_component_mut<C: Component>(&mut self) -> Vec<(Entity, &mut C)> {
        self.get_storage_mut::<C>()
            .map(|storage| storage.iter_mut().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestComponent(i32);
    impl Component for TestComponent {}

    #[test]
    fn test_entity_lifecycle() {
        let mut backend = SparseSetBackend::default();
        
        // Create entity
        let entity = backend.create_entity();
        assert!(backend.is_alive(entity));
        
        // Add component
        backend.add_component(entity, TestComponent(42)).unwrap();
        assert_eq!(backend.get_component::<TestComponent>(entity), Some(&TestComponent(42)));
        
        // Destroy entity
        backend.destroy_entity(entity).unwrap();
        assert!(!backend.is_alive(entity));
        assert_eq!(backend.get_component::<TestComponent>(entity), None);
    }

    #[test]
    fn test_entity_id_reuse() {
        let mut backend = SparseSetBackend::default();
        
        // Create and destroy entity
        let entity1 = backend.create_entity();
        let id1 = entity1.index();
        backend.destroy_entity(entity1).unwrap();
        
        // Create new entity - should reuse ID with new generation
        let entity2 = backend.create_entity();
        assert_eq!(entity2.index(), id1);
        assert_ne!(entity2.generation(), entity1.generation());
    }
}