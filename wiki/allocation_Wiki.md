# SigmaOS-allocation Component Wiki

## Overview
SigmaOS-allocation provides memory allocation and resource allocation support for the SigmaOS operating system. It demonstrates the allocation subsystem and memory management within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic allocation initialization.

## Principles
- Single Responsibility
- Allocation Abstraction
- Fail-Fast on Allocation Initialization Errors
- Deterministic Allocation Setup
- Clear Abstraction Over Allocation APIs
- Composition over Inheritance for allocation extensions

## Architecture
SigmaOS-allocation follows an allocation pattern:
- **Allocation Core**: Provides memory allocation, resource management, and allocation tracking
- **Memory Management**: Supports heap allocation, memory pooling, and allocation tracking
- **Resource Tracking**: Supports resource allocation, deallocation, and lifetime management
- **Error Handling**: Fail-fast on invalid allocation configurations or unsupported allocation types

## API
The component provides the following public APIs:
- `alloc::memory::allocate(size)` - Allocate memory of given size
- `alloc::memory::deallocate(ptr)` - Deallocate previously allocated memory
- `alloc::status::usage()` - Get current memory usage and allocation status
- `alloc::pool::create(pool_size)` - Create a memory pool with given size
- Error types for allocation failure scenarios or unsupported allocation types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific allocation configuration
2. **Act**: Execute the code under test (memory allocation, deallocation)
3. **Assert**: Verify the expected allocation results

Example:
```rust
#[test]
fn test_allocate_memory_aaa() {
    // Arrange
    let size = 1024;
    
    // Act
    let result = alloc::memory::allocate(size);
    
    // Assert
    assert!(result.is_ok());
    let allocated = result.unwrap();
    assert!(allocated.is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Allocation initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all allocation failure scenarios
- Unit tests provided for memory allocation and deallocation
- Integration tests for full allocation system with actual memory management

## TODO
- Add support for additional allocation types (heap, stack, pool)
- Expand memory management capabilities (garbage collection, memory profiling, memory leak detection)
- Add pool features (pool partitioning, pool merging, pool optimization)
- Create integration tests with actual memory management systems
- Implement allocation persistence and memory state management