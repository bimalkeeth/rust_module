#[macro_use]
extern crate criterion;
use main;

use criterion_demo::{fast_fibonacci, slow_fibonacci};
use criterion::Criterion;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 8", |b| b.iter(|| slow_fibonacci(8)));
}
criterion_group!(fib_bench, fibonacci_benchmark);
criterion_main!(fib_bench);