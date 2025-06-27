# VulkanContext Improvement Plan

## Overview
Transform VulkanContext from basic Vulkan initialization into a robust, configurable foundation for the Gamma-VK graphics engine.

## Implementation Checklist

### Phase 1: Core Functionality Enhancements (Priority: High)

#### 1.1 Graphics Queue Access
- [x] Add `graphics_queue()` method to expose the queue for command submission
- [x] Add `graphics_queue_family_index()` to get the queue family index  
- [x] Store queue reference during device creation in `new()`
- [x] Ensure thread-safe access with Arc wrapper
- [x] Update tests to verify queue access methods

#### 1.2 Device Selection Algorithm  
- [ ] ~~Create `select_best_physical_device()` helper method~~ (DEFERRED - M1 Mac only has integrated GPU)
- [ ] ~~Implement device scoring based on type (prefer discrete GPU)~~ (DEFERRED)
- [ ] ~~Score based on available features and extensions~~ (DEFERRED)
- [ ] ~~Check queue family capabilities in scoring~~ (DEFERRED)
- [ ] ~~Maintain fallback to first available device~~ (DEFERRED)
- [ ] ~~Add tests for device selection logic~~ (DEFERRED)

*Note: Device selection algorithm deferred as we're developing on M1 Mac with only integrated GPU available. Will implement when testing on systems with multiple GPUs.*

#### 1.3 Memory Allocator Integration
- [x] Add `StandardMemoryAllocator` creation in `new()`
- [x] Store allocator as field in VulkanContext
- [x] Expose allocator through `memory_allocator()` accessor
- [x] Update tests to verify allocator access
- [x] Document allocator usage patterns

### Phase 2: Builder Pattern API (Priority: Medium)

#### 2.1 VulkanContextBuilder
- [x] Create `VulkanContextBuilder` struct
- [x] Add `application_name()` method
- [x] Add `application_version()` method  
- [x] Add `engine_name()` and `engine_version()` methods
- [x] Add `prefer_discrete_gpu()` flag
- [x] Add `enable_validation_layers()` option
- [x] Add `required_extensions()` method (simplified - Vulkano doesn't support dynamic extensions)
- [x] Implement `build()` method
- [x] Update VulkanContext to use builder internally
- [x] Add builder pattern tests

#### 2.2 Validation Layer Support
- [ ] Auto-enable validation layers in debug builds
- [ ] Check layer availability before enabling
- [ ] Provide option to force enable/disable
- [ ] Set up debug messenger for validation messages
- [ ] Add tests for validation layer behavior

### Phase 3: Enhanced Error Handling (Priority: Medium)

#### 3.1 Actionable Error Messages
- [ ] Add `DeviceSelectionError` variant to GammaVkError
- [ ] Improve "No suitable GPU found" with driver check suggestion
- [ ] List available extensions when extension not supported
- [ ] Show device capabilities when queue family missing
- [ ] Add context to all error paths

#### 3.2 Fallback Strategies
- [ ] Implement graceful degradation for missing features
- [ ] Try multiple device selection strategies
- [ ] Log warnings for suboptimal configurations
- [ ] Document fallback behavior

### Phase 4: Extended Device Information (Priority: Low)

#### 4.1 Device Properties Access
- [ ] Add `supported_extensions()` method
- [ ] Add `device_properties()` method
- [ ] Add `memory_properties()` method
- [ ] Add `queue_family_properties()` method
- [ ] Add tests for property accessors

#### 4.2 Feature Detection
- [ ] Create `DeviceFeatures` struct
- [ ] Implement runtime feature checks
- [ ] Check for optional features (geometry shaders, tessellation, etc.)
- [ ] Provide easy API for feature queries
- [ ] Document available features

## Implementation Notes

### API Consistency
- All public methods take parameters as `&Arc<T>` (not `Arc<T>`)
- Internal implementation clones Arc only when needed
- Maintain RAII principles for all resources

### Testing Strategy
- Each feature must have corresponding tests
- Maintain backward compatibility
- Test error paths and fallback behavior
- Verify thread safety where applicable

### Documentation Requirements
- All public APIs need rustdoc with examples
- Document error conditions
- Explain fallback behaviors
- Include usage patterns

## Progress Tracking
Mark items with [x] as completed. Update this file as implementation progresses.