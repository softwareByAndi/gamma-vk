# Phase 1: ECS Foundation Implementation Plan

## Overview
This document outlines the step-by-step implementation to make our Phase 1 tests pass. We follow the Red-Green-Refactor cycle for each component.

## Implementation Order

### 1. Core Types and Traits (Entity, Component)

#### 1.1 Entity Type
```rust
// src/ecs/entity.rs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Entity {
    id: u32,
    generation: u32,
}

impl Entity {
    pub fn from_raw_parts(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }
    
    pub fn id(&self) -> u64 {
        ((self.generation as u64) << 32) | (self.id as u64)
    }
}
```

**Why this design?**
- Generational indices prevent ID reuse bugs
- Compact 64-bit representation
- Copy type for ergonomic use

#### 1.2 Component Trait
```rust
// src/ecs/component.rs
pub trait Component: Send + Sync + 'static {
    // Marker trait for now
}
```

**Why this design?**
- Send + Sync for thread safety
- 'static for type erasure in storage
- Marker trait allows future extension

### 2. ECS Backend Trait

```rust
// src/ecs/backend.rs
pub trait EcsBackend: Send + Sync {
    type EntityId: Copy + Eq + Hash + Debug;
    
    fn create_entity(&mut self) -> Self::EntityId;
    fn destroy_entity(&mut self, entity: Self::EntityId) -> Result<(), GammaVkError>;
    fn is_alive(&self, entity: Self::EntityId) -> bool;
    
    // Component operations use TypeId for type erasure
    fn add_component_raw(&mut self, entity: Self::EntityId, component: Box<dyn Any>) -> Result<(), GammaVkError>;
    fn get_component_raw(&self, entity: Self::EntityId, type_id: TypeId) -> Option<&dyn Any>;
    fn get_component_raw_mut(&mut self, entity: Self::EntityId, type_id: TypeId) -> Option<&mut dyn Any>;
    fn remove_component_raw(&mut self, entity: Self::EntityId, type_id: TypeId) -> Result<(), GammaVkError>;
}
```

### 3. Sparse Set Backend Implementation

#### 3.1 Storage Structure
```rust
// src/ecs/sparse_set_backend.rs
pub struct SparseSetBackend {
    // Entity management
    entities: Vec<EntityMetadata>,
    free_list: Vec<u32>,
    
    // Component storage: TypeId -> ComponentStorage
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

struct EntityMetadata {
    generation: u32,
    alive: bool,
}

trait ComponentStorage: Any + Send + Sync {
    fn remove(&mut self, entity: Entity) -> bool;
    fn clear_for_entity(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

#### 3.2 Type-Safe Component Storage
```rust
struct SparseSet<T: Component> {
    // Sparse array: entity ID -> dense index
    sparse: Vec<Option<usize>>,
    
    // Dense arrays
    entities: Vec<Entity>,
    components: Vec<T>,
}

impl<T: Component> ComponentStorage for SparseSet<T> {
    // Implement trait methods...
}
```

### 4. World API

#### 4.1 World Structure
```rust
// src/ecs/world.rs
pub struct World<B: EcsBackend = SparseSetBackend> {
    backend: B,
}

impl<B: EcsBackend> World<B> {
    pub fn new() -> Result<Self, GammaVkError> {
        Ok(Self {
            backend: B::default(),
        })
    }
    
    pub fn spawn(&mut self) -> EntityBuilder<B> {
        EntityBuilder::new(self)
    }
    
    // Type-safe wrappers around backend
    pub fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        self.backend
            .get_component_raw(entity, TypeId::of::<C>())
            .and_then(|any| any.downcast_ref::<C>())
    }
}
```

#### 4.2 Entity Builder
```rust
pub struct EntityBuilder<'a, B: EcsBackend> {
    world: &'a mut World<B>,
    entity: Entity,
}

impl<'a, B: EcsBackend> EntityBuilder<'a, B> {
    pub fn with<C: Component>(self, component: C) -> Self {
        self.world.add_component(self.entity, component).ok();
        self
    }
    
    pub fn build(self) -> Entity {
        self.entity
    }
}
```

### 5. Query System

#### 5.1 Query Trait
```rust
// src/ecs/query.rs
pub trait Query {
    type Item<'a>;
    
    fn fetch<'a>(world: &'a World) -> Vec<(Entity, Self::Item<'a>)>;
}

// Single component query
impl<C: Component> Query for &C {
    type Item<'a> = &'a C;
    
    fn fetch<'a>(world: &'a World) -> Vec<(Entity, Self::Item<'a>)> {
        // Iterate through component storage
    }
}

// Tuple queries for multiple components
impl<A: Component, B: Component> Query for (&A, &B) {
    type Item<'a> = (&'a A, &'a B);
    
    fn fetch<'a>(world: &'a World) -> Vec<(Entity, Self::Item<'a>)> {
        // Find entities with both components
    }
}
```

## Testing Strategy

### Test Order
1. **Entity creation and ID uniqueness** - Start here
2. **Component storage and retrieval** - Core functionality
3. **Entity destruction and cleanup** - RAII verification
4. **Query system** - Higher-level API
5. **Builder pattern** - Ergonomics

### Making Tests Pass

#### Step 1: Red Phase
- Run `cargo test --test phase1_foundation_tests`
- See compilation errors and test failures
- This defines our specification

#### Step 2: Green Phase
- Implement minimal code to compile
- Make tests pass one by one
- Don't worry about optimization yet

#### Step 3: Refactor Phase
- Improve code structure
- Add documentation
- Optimize hot paths
- Ensure RAII patterns

## Error Handling

Add to `src/error.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum GammaVkError {
    // ... existing variants ...
    
    #[error("Entity not found: {0:?}")]
    EntityNotFound(Entity),
    
    #[error("Component not found for entity: {0:?}")]
    ComponentNotFound(Entity),
}
```

## Performance Considerations

### Sparse Set Trade-offs
- **Fast**: O(1) component access
- **Fast**: O(1) add/remove component
- **Slower**: Iteration (not cache-friendly)
- **Memory**: Higher due to sparse arrays

### Optimization Opportunities (Phase 3)
1. Dense packing for iteration
2. Component pools to reduce allocation
3. Parallel query execution
4. Custom allocators

## Next Steps

1. Create `src/ecs/mod.rs` to define module structure
2. Implement types in order listed above
3. Run tests after each component
4. Document public API as we go
5. Create examples/ecs_demo.rs to validate ergonomics

## Success Criteria

- [ ] All Phase 1 tests pass
- [ ] No memory leaks (verified with Miri)
- [ ] Clean API that "feels right"
- [ ] Performance baseline established
- [ ] Ready for Phase 2 (multiple backends)