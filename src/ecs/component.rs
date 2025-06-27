//! Component trait and related types for the ECS system
//! 
//! Components are plain data types that can be attached to entities.
//! They must be Send + Sync for thread safety and 'static for type erasure.

use std::any::TypeId;

/// Trait that all components must implement.
/// 
/// Components are data containers that can be attached to entities.
/// This is a marker trait with supertraits for thread safety.
/// 
/// # Example
/// ```
/// #[derive(Debug, Clone)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
/// 
/// impl Component for Position {}
/// ```
pub trait Component: Send + Sync + 'static {}

/// Internal trait for type-erased component storage.
/// 
/// This allows us to store different component types in a single collection
/// while maintaining type safety through the public API.
pub(crate) trait ComponentStorage: Send + Sync {
    /// Removes a component for the given entity.
    fn remove(&mut self, entity: crate::Entity) -> bool;
    
    /// Clears all components for an entity (used during entity destruction).
    fn clear_for_entity(&mut self, entity: crate::Entity);
    
    /// Returns the type ID of the components stored.
    fn type_id(&self) -> TypeId;
    
    /// Converts to Any for downcasting.
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Converts to mutable Any for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestComponent {
        value: i32,
    }
    
    impl Component for TestComponent {}

    #[test]
    fn test_component_impl() {
        // This test just verifies that our Component trait can be implemented
        let component = TestComponent { value: 42 };
        
        // Component should be Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TestComponent>();
        
        // Should be able to get TypeId
        let type_id = TypeId::of::<TestComponent>();
        assert_ne!(type_id, TypeId::of::<i32>());
    }
}