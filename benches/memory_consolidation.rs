//! Performance benchmarks for memory consolidation operations
//!
//! Measures performance of memory system operations including:
//! - Short-term memory operations
//! - Memory consolidation cycles

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sentinel::core::types::{CanonicalMessage, Role};
use sentinel::memory::short_term::ShortTermMemory;

/// Benchmark short-term memory add operations
fn bench_short_term_add(c: &mut Criterion) {
    c.bench_function("short_term_add_single", |b| {
        let mut memory = ShortTermMemory::new();
        b.iter(|| {
            let msg = CanonicalMessage::new(Role::User, black_box("Test message".to_string()));
            memory.append_message(msg).unwrap();
        });
    });
}

/// Benchmark short-term memory batch operations
fn bench_short_term_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("short_term_batch");

    for size in [10, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut memory = ShortTermMemory::new();
                for i in 0..size {
                    let msg = CanonicalMessage::new(
                        if i % 2 == 0 {
                            Role::User
                        } else {
                            Role::Assistant
                        },
                        black_box(format!("Message {}", i)),
                    );
                    memory.append_message(msg).unwrap();
                }
                black_box(memory)
            });
        });
    }

    group.finish();
}

/// Benchmark short-term memory retrieval
fn bench_short_term_retrieve(c: &mut Criterion) {
    c.bench_function("short_term_retrieve_all", |b| {
        let mut memory = ShortTermMemory::new();
        // Add 100 messages
        for i in 0..100 {
            let msg = CanonicalMessage::new(
                if i % 2 == 0 {
                    Role::User
                } else {
                    Role::Assistant
                },
                format!("Message {}", i),
            );
            memory.append_message(msg).unwrap();
        }
        // Retrieve all
        b.iter(|| {
            let _messages = memory.get_messages();
        });
    });
}

/// Benchmark memory consolidation simulation
fn bench_consolidation_simulation(c: &mut Criterion) {
    c.bench_function("consolidation_100_messages", |b| {
        b.iter(|| {
            let mut memory = ShortTermMemory::new();
            // Add 100 messages to simulate consolidation trigger
            for i in 0..100 {
                let msg = CanonicalMessage::new(
                    if i % 2 == 0 {
                        Role::User
                    } else {
                        Role::Assistant
                    },
                    format!("Message {} with some content to make it longer", i),
                );
                memory.append_message(msg).unwrap();
            }
            // Simulate consolidation: get all, clear, would summarize
            let messages = memory.get_messages();
            memory.clear().unwrap();
            black_box(messages)
        });
    });
}

criterion_group!(
    benches,
    bench_short_term_add,
    bench_short_term_batch,
    bench_short_term_retrieve,
    bench_consolidation_simulation
);
criterion_main!(benches);
