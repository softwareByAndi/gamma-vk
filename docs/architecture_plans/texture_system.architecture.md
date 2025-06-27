# Texture System Architecture Plan

## Overview
The texture system provides type-safe, RAII-managed texture resources for Gamma-VK, supporting common formats and automatic mipmap generation with a focus on ease of use and performance.

## Behavior Specification

### Expected Behaviors (Test-First)
1. **Texture Loading**: Load textures from common image formats
   - Test: `test_texture_loads_png_successfully`
   - Edge case: File not found returns appropriate error

2. **Format Support**: Support common texture formats (RGBA, RGB, etc.)
   - Test: `test_texture_converts_rgb_to_rgba`
   - Edge case: Unsupported format returns error with suggestion

3. **Automatic Resource Management**: Textures clean up GPU memory when dropped
   - Test: `test_texture_memory_freed_on_drop`
   - Recovery: Memory allocation failures handled gracefully

4. **Mipmap Generation**: Automatically generate mipmaps when requested
   - Test: `test_texture_generates_correct_mip_levels`
   - Edge case: Non-power-of-two textures handled correctly

### Public API Design
```rust
// Key public interfaces that tests will verify
pub struct Texture {
    image: Arc<Image>,
    view: Arc<ImageView>,
    dimensions: (u32, u32),
    format: Format,
}

impl Texture {
    pub fn from_file(
        context: &VulkanContext,
        path: impl AsRef<Path>
    ) -> Result<Self, GammaVkError> {
        // Test first, implement after
    }
    
    pub fn from_bytes(
        context: &VulkanContext,
        bytes: &[u8],
        dimensions: (u32, u32),
        format: TextureFormat,
    ) -> Result<Self, GammaVkError> {
        // Test first, implement after
    }
    
    pub fn view(&self) -> &Arc<ImageView> {
        &self.view
    }
}
```

## Implementation Checklist

### Phase 1: Foundation (Priority: High)

#### 1.1 Core Texture Type
- [ ] Write test for texture creation from bytes
- [ ] Define Texture struct with RAII fields
- [ ] Implement from_bytes() with basic RGBA support
- [ ] Add TextureError variants to GammaVkError
- [ ] Document usage with examples

#### 1.2 Basic Image Loading
- [ ] Write test for PNG file loading
- [ ] Add image crate dependency
- [ ] Implement from_file() for PNG support
- [ ] Test file not found error handling
- [ ] Add format conversion (RGB→RGBA)

#### 1.3 GPU Memory Management
- [ ] Write test verifying memory cleanup
- [ ] Implement proper Image/ImageView creation
- [ ] Test memory allocation failure handling
- [ ] Verify RAII cleanup with drop test
- [ ] Document memory usage patterns

### Phase 2: Integration (Priority: Medium)

#### 2.1 Shader Integration
- [ ] Write test for texture binding in shaders
- [ ] Create sampler management
- [ ] Test descriptor set integration
- [ ] Implement layout transitions
- [ ] Document shader usage examples

#### 2.2 Format Support
- [ ] Write tests for additional formats (BC1-7, etc.)
- [ ] Implement format detection from file
- [ ] Add format compatibility checking
- [ ] Test format conversion paths
- [ ] Document supported formats table

### Phase 3: Performance (Priority: Low)

#### 3.1 Mipmap Generation
- [ ] Write test for mip level calculation
- [ ] Implement automatic mipmap generation
- [ ] Test non-power-of-two handling
- [ ] Benchmark generation performance
- [ ] Add manual mipmap loading option

#### 3.2 Texture Streaming
- [ ] Write test for async texture loading
- [ ] Implement background loading
- [ ] Add placeholder texture support
- [ ] Test concurrent loading
- [ ] Document streaming patterns

## Technical Considerations

### Dependencies
- Depends on: VulkanContext, memory allocator
- External: image crate for file loading
- Used by: Material system, renderer

### Platform Notes
- macOS/MoltenVK: Some compressed formats may not be supported
- Cross-platform: Verify format support per platform

### Risk Assessment
- **Main Risk**: Memory allocation failures under pressure
- **Mitigation**: Implement texture budget/eviction system in later iteration

## Definition of Done
- [ ] All tests pass
- [ ] Supports PNG, JPEG at minimum
- [ ] Automatic format conversion for common cases
- [ ] Memory freed automatically on drop
- [ ] No clippy warnings
- [ ] Loading benchmark under 50ms for 1024x1024
- [ ] Examples demonstrate common usage patterns