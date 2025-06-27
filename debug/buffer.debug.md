# Buffer Implementation Debug Notes

## Device Parameter (2025-06-26)
**Issue**: Thought unused `Arc<Device>` parameter should be removed  
**Reality**: Needed for future device validation and API consistency  
**Lesson**: Don't remove "unused" parameters without understanding architectural purpose

## Memory Type Requirements (2025-06-26)
**Issue**: Using `AllocationCreateInfo::default()` causes write failures  
**Reality**: Must specify memory type filter for host-visible access  
**Fix**: `MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE`  
**Pattern**: Host-visible vs device-local require different allocation strategies

## Buffer Usage Philosophy (2025-06-26)
**Issue**: Confused "buffer type" with "memory access pattern"  
**Reality**: Usage flags define GPU capabilities, memory type defines CPU access  
**Fix**: Separate `new_host_visible()` and `new_device_local()` constructors  
**Pattern**: `BufferUsage` ≠ memory accessibility

## Test Design Flaw (2025-06-26)
**Issue**: Testing implementation details instead of user requirements  
**Reality**: Tests should verify "Can I write vertex data?" not "Do zero buffers fail?"  
**Fix**: Focus tests on business requirements, not vulkano internals  
**Lesson**: Test what users care about, not implementation assumptions

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

## Vulkano Memory Allocation (2025-06-26)
**Host-visible**: `MemoryTypeFilter::PREFER_HOST | HOST_SEQUENTIAL_WRITE`  
**Device-local**: `MemoryTypeFilter::PREFER_DEVICE`  
**Key**: Memory type filter determines CPU accessibility, not buffer usage flags

## Error Handling (2025-06-26)
**Pattern**: Domain-specific errors with context
```rust
GammaVkError::buffer_creation(format!("Failed to create host-visible buffer: {}", e))
```