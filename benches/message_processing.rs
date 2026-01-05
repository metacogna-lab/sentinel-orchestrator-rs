//! Performance benchmarks for message processing operations
//!
//! Measures performance of core message operations including:
//! - Single message processing
//! - Batch message processing
//! - Message validation
//! - CanonicalMessage creation

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sentinel::core::types::{CanonicalMessage, Role};

/// Benchmark single message creation
fn bench_single_message_creation(c: &mut Criterion) {
    c.bench_function("create_single_message", |b| {
        b.iter(|| {
            let _msg =
                CanonicalMessage::new(Role::User, black_box("Test message content".to_string()));
        });
    });
}

/// Benchmark batch message creation
fn bench_batch_message_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_message_creation");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut messages = Vec::with_capacity(size);
                for i in 0..size {
                    messages.push(CanonicalMessage::new(
                        if i % 2 == 0 {
                            Role::User
                        } else {
                            Role::Assistant
                        },
                        black_box(format!("Message {}", i)),
                    ));
                }
                black_box(messages)
            });
        });
    }

    group.finish();
}

/// Benchmark message serialization
fn bench_message_serialization(c: &mut Criterion) {
    let message = CanonicalMessage::new(
        Role::User,
        "Test message content for serialization benchmark".to_string(),
    );

    c.bench_function("serialize_message", |b| {
        b.iter(|| {
            let _json = serde_json::to_string(black_box(&message)).unwrap();
        });
    });

    c.bench_function("deserialize_message", |b| {
        let json = serde_json::to_string(&message).unwrap();
        b.iter(|| {
            let _msg: CanonicalMessage = serde_json::from_str(black_box(&json)).unwrap();
        });
    });
}

/// Benchmark message validation overhead
fn bench_message_validation(c: &mut Criterion) {
    let valid_message = CanonicalMessage::new(Role::User, "Valid message content".to_string());

    c.bench_function("validate_message", |b| {
        b.iter(|| {
            // Message validation checks (content not empty, valid role, etc.)
            let _is_valid = !black_box(&valid_message).content.is_empty()
                && matches!(
                    black_box(&valid_message).role,
                    Role::User | Role::Assistant | Role::System
                );
        });
    });
}

criterion_group!(
    benches,
    bench_single_message_creation,
    bench_batch_message_creation,
    bench_message_serialization,
    bench_message_validation
);
criterion_main!(benches);
