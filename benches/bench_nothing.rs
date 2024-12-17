use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn bench_factorial(c: &mut Criterion) {
    c.bench_function("factorial 10", |b| b.iter(|| factorial(black_box(10))));
}

criterion_group!(benches, criterion_benchmark, bench_factorial);
criterion_main!(benches);   
