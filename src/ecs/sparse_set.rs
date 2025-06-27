//! Sparse set storage for components
//! 
//! Provides O(1) insertion, removal, and access at the cost of memory overhead
//! and less cache-friendly iteration compared to archetype storage.

use crate::{Component, ComponentStorage, Entity};
use std::any::{Any, TypeId};

/// A sparse set data structure for storing components.
/// 
/// Uses a sparse array for O(1) entity -> component lookup
/// and a dense array for cache-friendly iteration.
pub(crate) struct SparseSet<T: Component> {
    /// Sparse array: entity index -> dense index
    sparse: Vec<Option<usize>>,
    
    /// Dense array of entities (parallel to components)
    entities: Vec<Entity>,
    
    /// Dense array of components (parallel to entities)
    components: Vec<T>,
}

impl<T: Component> SparseSet<T> {
    /// Creates a new empty sparse set.
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            entities: Vec::new(),
            components: Vec::new(),
        }
    }
    
    /// Inserts a component for an entity.
    pub fn insert(&mut self, entity: Entity, component: T) {
        let index = entity.index() as usize;
        
        // Grow sparse array if needed
        if index >= self.sparse.len() {
            self.sparse.resize(index + 1, None);
        }
        
        // Check if entity already has component
        if let Some(dense_index) = self.sparse[index] {
            // Update existing component
            self.components[dense_index] = component;
            self.entities[dense_index] = entity; // Update generation
        } else {
            // Add new component
            let dense_index = self.components.len();
            self.sparse[index] = Some(dense_index);
            self.entities.push(entity);
            self.components.push(component);
        }
    }
    
    /// Gets a component for an entity.
    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = entity.index() as usize;
        
        self.sparse
            .get(index)
            .and_then(|&dense_index| dense_index)
            .and_then(|dense_index| {
                // Verify generation matches
                if self.entities[dense_index] == entity {
                    Some(&self.components[dense_index])
                } else {
                    None
                }
            })
    }
    
    /// Gets a mutable component for an entity.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = entity.index() as usize;
        
        self.sparse
            .get(index)
            .and_then(|&dense_index| dense_index)
            .and_then(|dense_index| {
                // Verify generation matches
                if self.entities[dense_index] == entity {
                    Some(&mut self.components[dense_index])
                } else {
                    None
                }
            })
    }
    
    /// Removes a component for an entity.
    pub fn remove(&mut self, entity: Entity) -> bool {
        let index = entity.index() as usize;
        
        if let Some(Some(dense_index)) = self.sparse.get(index) {
            // Verify generation matches
            if self.entities[*dense_index] != entity {
                return false;
            }
            
            // Swap remove from dense arrays
            let last_index = self.components.len() - 1;
            
            if *dense_index != last_index {
                self.entities.swap(*dense_index, last_index);
                self.components.swap(*dense_index, last_index);
                
                // Update sparse array for swapped entity
                let swapped_entity_index = self.entities[*dense_index].index() as usize;
                self.sparse[swapped_entity_index] = Some(*dense_index);
            }
            
            // Remove last element
            self.entities.pop();
            self.components.pop();
            self.sparse[index] = None;
            
            true
        } else {
            false
        }
    }
    
    /// Iterates over all entities and components.
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities.iter().copied()
            .zip(self.components.iter())
    }
    
    /// Iterates over all entities and mutable components.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.entities.iter().copied()
            .zip(self.components.iter_mut())
    }
}

impl<T: Component> ComponentStorage for SparseSet<T> {
    fn remove(&mut self, entity: Entity) -> bool {
        self.remove(entity)
    }
    
    fn clear_for_entity(&mut self, entity: Entity) {
        self.remove(entity);
    }
    
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestComponent(i32);
    impl Component for TestComponent {}

    #[test]
    fn test_sparse_set_insert_and_get() {
        let mut storage = SparseSet::<TestComponent>::new();
        let entity = Entity::from_raw_parts(5, 1);
        
        storage.insert(entity, TestComponent(42));
        
        assert_eq!(storage.get(entity), Some(&TestComponent(42)));
    }

    #[test]
    fn test_sparse_set_update() {
        let mut storage = SparseSet::<TestComponent>::new();
        let entity = Entity::from_raw_parts(3, 1);
        
        storage.insert(entity, TestComponent(10));
        storage.insert(entity, TestComponent(20));
        
        assert_eq!(storage.get(entity), Some(&TestComponent(20)));
    }

    #[test]
    fn test_sparse_set_remove() {
        let mut storage = SparseSet::<TestComponent>::new();
        let entity = Entity::from_raw_parts(2, 1);
        
        storage.insert(entity, TestComponent(5));
        assert!(storage.remove(entity));
        assert_eq!(storage.get(entity), None);
        assert!(!storage.remove(entity)); // Second remove fails
    }

    #[test]
    fn test_sparse_set_generation_check() {
        let mut storage = SparseSet::<TestComponent>::new();
        let entity_gen1 = Entity::from_raw_parts(1, 1);
        let entity_gen2 = Entity::from_raw_parts(1, 2);
        
        storage.insert(entity_gen1, TestComponent(100));
        
        // Different generation should not find component
        assert_eq!(storage.get(entity_gen2), None);
    }

    #[test]
    fn test_sparse_set_iteration() {
        let mut storage = SparseSet::<TestComponent>::new();
        
        let e1 = Entity::from_raw_parts(1, 1);
        let e2 = Entity::from_raw_parts(5, 1);
        let e3 = Entity::from_raw_parts(3, 1);
        
        storage.insert(e1, TestComponent(1));
        storage.insert(e2, TestComponent(2));
        storage.insert(e3, TestComponent(3));
        
        let mut results: Vec<_> = storage.iter()
            .map(|(e, c)| (e, c.0))
            .collect();
        results.sort_by_key(|(_, val)| *val);
        
        assert_eq!(results, vec![(e1, 1), (e2, 2), (e3, 3)]);
    }
}