use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_app_exemplo::*;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci_optimized(black_box(20)))
    });

    c.bench_function("fibonacci 30", |b| {
        b.iter(|| fibonacci_optimized(black_box(30)))
    });
}

fn factorial_benchmark(c: &mut Criterion) {
    c.bench_function("factorial 10", |b| b.iter(|| factorial(black_box(10))));

    c.bench_function("factorial 20", |b| b.iter(|| factorial(black_box(20))));
}

fn prime_benchmark(c: &mut Criterion) {
    c.bench_function("is_prime 1000", |b| b.iter(|| is_prime(black_box(1000))));

    c.bench_function("is_prime 10007", |b| b.iter(|| is_prime(black_box(10007))));
}

fn string_utils_benchmark(c: &mut Criterion) {
    c.bench_function("title_case", |b| {
        b.iter(|| string_utils::to_title_case(black_box("hello world from rust")))
    });

    c.bench_function("count_vowels", |b| {
        b.iter(|| {
            string_utils::count_vowels(black_box("the quick brown fox jumps over the lazy dog"))
        })
    });

    c.bench_function("reverse_string", |b| {
        b.iter(|| string_utils::reverse(black_box("abcdefghijklmnopqrstuvwxyz")))
    });
}

fn user_operations_benchmark(c: &mut Criterion) {
    c.bench_function("user_creation", |b| {
        b.iter(|| {
            User::new(
                black_box(1),
                black_box("John Doe".to_string()),
                black_box("john@example.com".to_string()),
            )
        })
    });

    c.bench_function("user_serialization", |b| {
        let user = User::new(1, "John Doe".to_string(), "john@example.com".to_string());
        b.iter(|| serde_json::to_string(black_box(&user)))
    });
}

criterion_group!(
    benches,
    fibonacci_benchmark,
    factorial_benchmark,
    prime_benchmark,
    string_utils_benchmark,
    user_operations_benchmark
);
criterion_main!(benches);
