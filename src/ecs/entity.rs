//! Entity type for the ECS system
//! 
//! Entities use generational indices to prevent ID reuse bugs.
//! Each entity has an ID and a generation counter.

use std::fmt;

/// An entity in the ECS world.
/// 
/// Entities are lightweight IDs that can have components attached to them.
/// They use generational indices to detect use-after-free scenarios.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Entity {
    /// The entity's unique identifier within its generation
    id: u32,
    /// Generation counter to detect stale entity references
    generation: u32,
}

impl Entity {
    /// Creates an entity from raw parts.
    /// 
    /// # Safety
    /// This is primarily for testing. Normal entity creation should go through World::spawn()
    pub fn from_raw_parts(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }
    
    /// Returns a unique 64-bit identifier combining ID and generation.
    /// 
    /// This is useful for external systems that need a single unique value.
    pub fn id(&self) -> u64 {
        ((self.generation as u64) << 32) | (self.id as u64)
    }
    
    /// Returns the entity's index (without generation).
    pub(crate) fn index(&self) -> u32 {
        self.id
    }
    
    /// Returns the entity's generation.
    pub(crate) fn generation(&self) -> u32 {
        self.generation
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({}, gen: {})", self.id, self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::from_raw_parts(42, 1);
        assert_eq!(entity.index(), 42);
        assert_eq!(entity.generation(), 1);
    }

    #[test]
    fn test_entity_id_packing() {
        let entity = Entity::from_raw_parts(100, 5);
        let packed_id = entity.id();
        
        // Verify packing
        assert_eq!(packed_id, (5u64 << 32) | 100);
        
        // Different entities have different IDs
        let entity2 = Entity::from_raw_parts(100, 6);
        assert_ne!(entity.id(), entity2.id());
    }

    #[test]
    fn test_entity_equality() {
        let e1 = Entity::from_raw_parts(1, 1);
        let e2 = Entity::from_raw_parts(1, 1);
        let e3 = Entity::from_raw_parts(1, 2);
        let e4 = Entity::from_raw_parts(2, 1);
        
        assert_eq!(e1, e2);
        assert_ne!(e1, e3); // Different generation
        assert_ne!(e1, e4); // Different ID
    }
}