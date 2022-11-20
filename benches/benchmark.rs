use advent_of_code_2022::advent_task::{dummy_task::DummyTask, AdventTask};
use criterion::{criterion_group, criterion_main, Criterion};

fn run_dummy() {
    DummyTask {}.solve();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Dummy task", |b| b.iter(|| run_dummy()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
