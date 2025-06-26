# Vulkano API Debug Notes

## Buffer Creation Pattern (2025-06-26)
```rust
Buffer::new_slice<T>(
    allocator: Arc<dyn MemoryAllocator>,  // Device context flows through allocator
    create_info: BufferCreateInfo,
    allocation_info: AllocationCreateInfo,  // CRITICAL: Must specify memory type!
    len: DeviceSize,  // u64, not usize!
) -> Result<Subbuffer<[T]>, AllocateBufferError>
```
**Key**: Returns `Subbuffer<[T]>`, device embedded in allocator

## Memory Type Filter Patterns (2025-06-26)
```rust
// Host-visible (CPU writable)
AllocationCreateInfo {
    memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    ..Default::default()
}

// Device-local (GPU optimal)
AllocationCreateInfo {
    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
    ..Default::default()
}
```
**Critical**: `AllocationCreateInfo::default()` often fails for host-visible buffers

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
- `MemoryTypeFilter` = determines CPU/GPU memory access patterns
- `Arc<dyn MemoryAllocator>` = shared memory allocator

## Shader Module Patterns (2024-06-26)

### VulkanoShaderModule Creation
```rust
// Requires OWNED Arc<Device>, not reference!
let module = unsafe { 
    VulkanoShaderModule::new(
        device.clone(),  // Must clone from &Arc<Device>
        ShaderModuleCreateInfo::new(&spirv_words)
    )
}?;
```
**Key**: Even if our API takes `&Arc<Device>`, Vulkano needs ownership

### SPIR-V Validation Requirements
```rust
// 1. Check byte alignment
if spirv_bytes.len() % 4 != 0 {
    return Err("SPIR-V must be 4-byte aligned");
}

// 2. Check magic number (0x07230203)
let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
if magic != 0x07230203 {
    return Err("Invalid SPIR-V magic number");
}

// 3. Convert to u32 words for Vulkano
let words: Vec<u32> = spirv_bytes
    .chunks_exact(4)
    .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    .collect();
```

### File vs Bytecode Loading
```rust
// File-based (preferred for development)
pub fn from_spirv_file(device: &Arc<Device>, path: impl AsRef<Path>) -> Result<Self> {
    let bytes = fs::read(path)?;
    Self::from_spirv_bytes(device, &bytes)
}

// Bytecode (for embedded or generated shaders)
pub fn from_spirv_bytes(device: &Arc<Device>, spirv_bytes: &[u8]) -> Result<Self> {
    // Validation and conversion...
}
```
**Lesson**: File-based is cleaner but both patterns have uses