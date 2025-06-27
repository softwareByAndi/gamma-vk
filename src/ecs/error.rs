//! Error types for the ECS system
//! 
//! These will be integrated into GammaVkError in the main codebase

use crate::Entity;
use thiserror::Error;

/// ECS-specific error types
#[derive(Debug, Error)]
pub enum EcsError {
    #[error("Entity not found: {0}")]
    EntityNotFound(Entity),
    
    #[error("Component not found for entity: {0}")]
    ComponentNotFound(Entity),
    
    #[error("Entity {0} is not alive")]
    EntityNotAlive(Entity),
}

// In the real implementation, these would be added to GammaVkError:
// #[derive(Debug, Error)]
// pub enum GammaVkError {
//     ... existing variants ...
//     
//     #[error("Entity not found: {0}")]
//     EntityNotFound(Entity),
//     
//     #[error("Component not found for entity: {0}")]
//     ComponentNotFound(Entity),
// }