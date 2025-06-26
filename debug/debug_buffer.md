# Buffer Implementation Debug Notes

## Device Parameter (2025-06-26)
**Issue**: Thought unused `Arc<Device>` parameter should be removed  
**Reality**: Needed for future device validation and API consistency  
**Lesson**: Don't remove "unused" parameters without understanding architectural purpose

## Vulkano Buffer API (2025-06-26)
**Pattern**: `Buffer::new_slice(allocator, create_info, allocation_info, size)`  
**Key**: Device embedded in allocator, returns `Subbuffer<[T]>` not `Arc<Buffer>`  
**Gotcha**: Size parameter is `u64` (DeviceSize) not `usize`

## Type Mismatch (2025-06-26)
**Issue**: `data.len()` (usize) vs `buffer.len()` (u64) comparison error  
**Fix**: `if data.len() > self.buffer.len() as usize`  
**Pattern**: Always cast when comparing slice lengths with device sizes

## RAII Pattern (2025-06-26)
**Choice**: `Subbuffer<[u8]>` over `Arc<Buffer>` for automatic cleanup  
**Benefit**: Built-in memory mapping and RAII resource management  
**Code**: `src/buffer.rs:20-23`

## Error Handling (2025-06-26)
**Pattern**: Domain-specific errors with helper methods
```rust
GammaVkError::BufferCreation { message: String }
impl GammaVkError {
    pub fn buffer_creation<S: Into<String>>(message: S) -> Self
}
```