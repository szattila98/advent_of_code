use advent_of_code_2022::advent_task::{
    dummy_task::DummyTask, elven_calories::ElvenCalories, AdventTask,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn run_dummy() {
    DummyTask {}.solve();
}

fn run_elven_calories() {
    ElvenCalories {}.solve();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Dummy task", |b| b.iter(|| run_dummy()));
    c.bench_function("Elven calories", |b| b.iter(|| run_elven_calories()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
