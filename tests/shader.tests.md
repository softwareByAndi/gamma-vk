# Shader Module Test Plan

This document outlines the comprehensive test cases for the ShaderModule functionality in Gamma-VK. These tests follow TDD principles and define the expected behavior before implementation.

## Test Categories

### 1. Unit Tests - Core Shader Loading

#### SPIR-V Bytecode Validation
- [x] `test_valid_spirv_magic_number` - Accepts valid SPIR-V magic number (0x07230203)
- [x] `test_invalid_spirv_magic_number` - Rejects invalid magic numbers with clear error
- [x] `test_spirv_bytecode_too_short` - Rejects bytecode shorter than 4 bytes
- [x] `test_spirv_bytecode_misaligned` - Rejects bytecode not aligned to 4 bytes
- [x] `test_empty_spirv_bytecode` - Handles empty byte arrays appropriately

#### Shader Module Creation
- [x] `test_from_spirv_bytes_success` - Creates module from valid SPIR-V bytes (part of test_valid_spirv_magic_number)
- [x] `test_from_spirv_bytes_minimal` - Works with minimal valid SPIR-V (part of test_valid_spirv_magic_number)
- [x] `test_vulkano_module_access` - Provides access to underlying Vulkano module (tested in multiple tests)
- [x] `test_shader_module_debug_format` - Debug implementation doesn't expose internals

### 2. Integration Tests - File Loading

#### File Operations
- [x] `test_from_spirv_file_success` - Loads valid shader file successfully
- [x] `test_from_spirv_file_missing` - Returns appropriate error for missing files
- [ ] `test_from_spirv_file_permission_denied` - Handles permission errors gracefully
- [ ] `test_from_spirv_file_invalid_path` - Handles invalid path characters

#### Common Shader Loading
- [x] `test_load_triangle_vertex_shader` - Loads common vertex shader if present
- [x] `test_load_triangle_fragment_shader` - Loads common fragment shader if present
- [x] `test_common_shaders_missing` - Handles missing common shaders appropriately (tested implicitly)

### 3. Resource Management Tests

#### RAII Behavior
- [x] `test_shader_module_drop_cleanup` - Verifies automatic resource cleanup
- [x] `test_multiple_shader_references` - Arc reference counting works correctly
- [x] `test_shader_module_thread_safety` - Can be safely shared across threads

### 4. Error Handling Tests

#### Error Types and Messages
- [x] `test_shader_compilation_error_type` - Returns correct error variant
- [x] `test_error_messages_descriptive` - Error messages are helpful and actionable
- [x] `test_error_context_preservation` - Original error context is preserved (tested implicitly)

### 5. Edge Cases and Boundaries

#### Size and Content Variations
- [ ] `test_large_spirv_file` - Handles large shader files (>1MB)
- [ ] `test_unicode_file_paths` - Supports non-ASCII file paths
- [ ] `test_relative_vs_absolute_paths` - Works with both path types
- [ ] `test_symlink_shader_files` - Follows symbolic links correctly

### 6. Platform-Specific Tests

#### Cross-Platform Behavior
- [ ] `test_windows_path_separators` - Handles Windows path separators
- [ ] `test_macos_case_sensitivity` - Respects filesystem case sensitivity
- [ ] `test_linux_hidden_files` - Can load hidden files (.filename)

## Test Implementation Notes

### Test Data Requirements
- Need minimal valid SPIR-V files for testing ✅
- Should create test fixtures with known-good shaders ✅
- Consider generating invalid SPIR-V for negative tests ✅

### Test Helpers Needed
- `create_test_context()` - Creates a test Vulkan context ✅
- `minimal_spirv_header()` - Returns minimal valid SPIR-V ✅
- `load_test_shader_bytes()` - Loads real shader files when available ✅

### Skip Conditions
- Skip file tests if test shaders don't exist
- Skip device tests if Vulkan unavailable
- Print informative messages when skipping

## Future Test Considerations

As the shader system evolves, consider adding:
- Shader reflection tests
- Pipeline integration tests
- Shader hot-reloading tests
- Shader caching tests
- Multi-stage shader tests

## Quality Criteria

All tests must be:
- Fast (<100ms per test)
- Deterministic (no timing dependencies)
- Isolated (no shared state)
- Descriptive (clear test names)
- Focused (one behavior per test)