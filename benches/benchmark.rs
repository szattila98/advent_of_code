use advent_of_code_2022::advent_task::{
    camp_cleaning::CampCleaning, device_cleanup::DeviceCleanup, elven_calories::ElvenCalories,
    elven_tournament::ElvenTournament, rucksack_troubles::RucksackTroubles,
    supply_stacks::SupplyStacks, tuning_trouble::TuningTrouble, AdventTask,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let task = ElvenCalories;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (67016, 200116)))
    });
    let task = ElvenTournament;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (13809, 12316)))
    });
    let task = RucksackTroubles;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (8252, 2828)))
    });
    let task = CampCleaning;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (459, 779)))
    });
    let task = SupplyStacks;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| {
            assert_eq!(
                task.solve(),
                ("WHTLRMZRC".to_string(), "GMPMLWNMG".to_string())
            )
        })
    });
    let task = TuningTrouble;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (1909, 3380)))
    });
    let task = DeviceCleanup;
    c.bench_function(task.get_name(), |b| {
        b.iter(|| assert_eq!(task.solve(), (1909, 2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
