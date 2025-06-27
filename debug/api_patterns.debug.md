# API Pattern Debug Notes

## Arc Reference Pattern (2024-06-26)

### Discovery
**Issue**: Inconsistent API - some functions took `Arc<T>`, others took `&Arc<T>`  
**Problem**: Forces unnecessary clones, especially in graphics loops  
**Solution**: Standardize on `&Arc<T>` for all public APIs

### The Pattern
```rust
// ❌ Bad: Forces clones
pub fn new(device: Arc<Device>, allocator: Arc<StandardMemoryAllocator>) -> Result<Self>

// ✅ Good: Flexible for callers  
pub fn new(device: &Arc<Device>, allocator: &Arc<StandardMemoryAllocator>) -> Result<Self>
```

### Why This Matters
```rust
// With Arc<T> parameters - lots of clones!
let vb1 = VertexBuffer::new(device.clone(), allocator.clone(), 1024)?;
let vb2 = VertexBuffer::new(device.clone(), allocator.clone(), 2048)?;
let ib = IndexBuffer::new(device.clone(), allocator.clone(), 512)?;
// 6 unnecessary Arc clones!

// With &Arc<T> parameters - clean!
let vb1 = VertexBuffer::new(&device, &allocator, 1024)?;
let vb2 = VertexBuffer::new(&device, &allocator, 2048)?;  
let ib = IndexBuffer::new(&device, &allocator, 512)?;
// Zero clones needed!
```

### When to Clone
```rust
pub fn new(device: &Arc<Device>, allocator: &Arc<StandardMemoryAllocator>) -> Result<Self> {
    // Clone ONLY when Vulkano needs ownership
    let buffer = VulkanoBuffer::new_slice(
        allocator.clone(),  // Vulkano requires Arc ownership
        ...
    )?;
}
```

### Performance Impact
- Each `Arc::clone()` increments atomic reference count
- In graphics loops creating many resources, this adds up
- `&Arc<T>` avoids ref count changes entirely
- Only clone when actually storing the Arc

### API Design Principles
1. **Take references by default**: `&Arc<T>` in parameters
2. **Clone only when storing**: Inside implementation if needed
3. **Be consistent**: All public APIs should follow same pattern
4. **Document ownership**: Make it clear when ownership transfers

### Real-World Example
```rust
// Graphics loop creating resources
for mesh in &meshes {
    // With Arc<T>: 2 clones per mesh
    // With &Arc<T>: 0 clones per mesh
    let buffer = VertexBuffer::new(&device, &allocator, mesh.size)?;
}
// 1000 meshes = 2000 clones saved!
```

### Gotcha: Internal Cloning Still Needed
```rust
// Even with &Arc parameter, Vulkano APIs may need owned Arc
let module = unsafe { 
    VulkanoShaderModule::new(device.clone(), create_info)  // Must clone here
}?;
```

### Summary
- Public API: Take `&Arc<T>`
- Internal: Clone when needed for Vulkano
- Result: Flexible, performant, consistent