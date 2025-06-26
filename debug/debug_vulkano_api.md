# Vulkano API Debug Notes

## Buffer Creation Pattern (2025-06-26)
```rust
Buffer::new_slice<T>(
    allocator: Arc<dyn MemoryAllocator>,  // Device context flows through allocator
    create_info: BufferCreateInfo,
    allocation_info: AllocationCreateInfo,
    len: DeviceSize,  // u64, not usize!
) -> Result<Subbuffer<[T]>, AllocateBufferError>
```
**Key**: Returns `Subbuffer<[T]>`, device embedded in allocator

## MoltenVK Compatibility (2025-06-26)
```rust
// Try portability first, fallback to standard
InstanceCreateInfo {
    enabled_extensions: InstanceExtensions {
        khr_portability_enumeration: true,
        ..InstanceExtensions::empty()
    },
    flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
    ..Default::default()
}
```

## Allocator Pattern (2025-06-26)
```rust
let allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
```
**Must**: Wrap in `Arc` for sharing, device context embedded

## Common Types
- `DeviceSize` = `u64` (device memory sizes)
- `Subbuffer<[T]>` = typed buffer slice with auto cleanup
- `Arc<dyn MemoryAllocator>` = shared memory allocator