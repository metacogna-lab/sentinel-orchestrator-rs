# Task 16: Performance Benchmarks

## Overview

Implement comprehensive performance benchmarks using `criterion` to measure and track performance metrics for critical system operations.

## Dependencies

**REQUIRES:**
- ✅ **Task 13** - API route handlers implemented
- ✅ **Task 14** - OpenAPI schema generated
- ✅ **Task 15** - End-to-end API integration tests (in progress)
- ✅ **Phase 4** - Memory system complete
- ✅ **Phase 5** - API layer complete

## Objectives

1. Set up `criterion` benchmark suite
2. Benchmark message processing performance
3. Benchmark memory consolidation performance
4. Benchmark API response time
5. Establish performance baselines
6. Track performance regressions

## Implementation Tasks

### 1. Add Criterion Dependency

**Location**: `Cargo.toml`

**Requirements**:
- Add `criterion` to `[dev-dependencies]`
- Configure benchmark harness

**Code Structure**:
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports", "async_tokio"] }

[[bench]]
name = "message_processing"
harness = false

[[bench]]
name = "memory_consolidation"
harness = false

[[bench]]
name = "api_response"
harness = false
```

### 2. Message Processing Benchmarks

**Location**: `benches/message_processing.rs` (new file)

**Test Cases**:
1. ✅ Benchmark single message processing
2. ✅ Benchmark batch message processing (10, 100, 1000 messages)
3. ✅ Benchmark message validation overhead
4. ✅ Benchmark CanonicalMessage creation
5. ✅ Target: < 100ms p99 for single message

**Code Structure**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sentinel::core::types::{CanonicalMessage, Role};

fn bench_message_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_processing");
    
    // Single message
    group.bench_function("single_message", |b| {
        b.iter(|| {
            let msg = CanonicalMessage::new(
                Role::User,
                black_box("Test message".to_string())
            );
            // Process message
        });
    });
    
    // Batch processing
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| {
                    // Process batch of messages
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_message_processing);
criterion_main!(benches);
```

### 3. Memory Consolidation Benchmarks

**Location**: `benches/memory_consolidation.rs` (new file)

**Test Cases**:
1. ✅ Benchmark short-term memory operations
2. ✅ Benchmark medium-term memory (Sled) operations
3. ✅ Benchmark long-term memory (Qdrant) operations
4. ✅ Benchmark full consolidation cycle
5. ✅ Target: < 5s for 100 messages

**Code Structure**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_memory_consolidation(c: &mut Criterion) {
    // Benchmark consolidation with different message counts
    // Use tempfile for Sled, mock for Qdrant
}
```

### 4. API Response Time Benchmarks

**Location**: `benches/api_response.rs` (new file)

**Test Cases**:
1. ✅ Benchmark health check endpoint
2. ✅ Benchmark chat completion endpoint (with mock LLM)
3. ✅ Benchmark agent status endpoint
4. ✅ Benchmark authentication middleware overhead
5. ✅ Target: < 200ms p99 for API response

**Code Structure**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sentinel::api::routes::create_router;
// ... setup test router with mock LLM

fn bench_api_response(c: &mut Criterion) {
    // Benchmark each endpoint
}
```

### 5. Performance Baseline Documentation

**Location**: `docs/performance.md` (new file)

**Requirements**:
- Document performance targets
- Document benchmark results
- Document how to run benchmarks
- Document performance regression detection

## Testing Requirements

### Benchmark Execution

**Commands**:
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench message_processing

# Compare with previous run
cargo bench -- --save-baseline baseline
cargo bench -- --baseline baseline
```

### Performance Targets

From PRD Section 7.4.2.4:
- Message processing: < 100ms p99
- Memory consolidation: < 5s for 100 messages
- API response time: < 200ms p99

## Acceptance Criteria

- [ ] Criterion benchmark suite set up
- [ ] All three benchmark files created and working
- [ ] Benchmarks run successfully: `cargo bench`
- [ ] Performance targets documented
- [ ] Baseline results recorded
- [ ] CI integration for performance regression detection (optional)
- [ ] Documentation in `docs/performance.md`
- [ ] No clippy warnings
- [ ] Code formatted

## Error Handling

- Benchmarks should handle errors gracefully
- Use `black_box` to prevent compiler optimizations
- Use appropriate sample sizes for statistical significance

## References

- PRD Section 7.4.2.4: Performance Optimization (lines 728-733)
- Criterion Documentation: https://github.com/bheisler/criterion.rs
- Architecture Doc: Performance considerations

## Next Task

After completing this task, proceed to **Phase 6 completion review** and update bridge.md

