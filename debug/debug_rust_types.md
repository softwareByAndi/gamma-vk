# Rust Type System Debug Notes

## Size Type Mismatch (2025-06-26)
**Issue**: `DeviceSize` (u64) vs `usize` in comparisons  
**Error**: `if data.len() > self.buffer.len()` // usize vs u64  
**Fix**: `if data.len() > self.buffer.len() as usize`  
**Why**: GPU devices use 64-bit sizes, host slices use pointer-sized indices

## Arc vs Reference Patterns (2025-06-26)
**Shared Resources**: `Arc<Device>`, `Arc<StandardMemoryAllocator>`  
**Temporary Access**: `&[u8]`, `&str` for function parameters  
**Rule**: Use `Arc<T>` for ownership sharing, `&T` for borrowing

## Generic Type Patterns (2025-06-26)
**Vulkano**: `Buffer::new_slice<T>()` creates `Subbuffer<[T]>`  
**Key**: Type parameter determines element type and return type  
**Usage**: `Buffer::new_slice::<u8>()` or let compiler infer

## Result Propagation (2025-06-26)
**Pattern**: Chain operations with `?` operator  
**Conversion**: Use `map_err()` to convert between error types  
```rust
vulkano_op().map_err(|e| GammaVkError::buffer_creation(format!("...: {}", e)))?
```

## Ownership in Constructors (2025-06-26)
**Builder Pattern**: `mut self` → `self` for method chaining  
**Resource Transfer**: APIs that take ownership vs borrowing  
**Vulkano**: Allocator ownership transferred to buffer creation