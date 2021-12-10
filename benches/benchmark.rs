use {
    aoc_2021::run_with,
    criterion::{criterion_group, criterion_main, Criterion},
};

fn criterion_benchmark(c: &mut Criterion) {
    (1..=10).for_each(|day| {
        c.bench_function(&format!("run_day_{}", day), |b| {
            b.iter(|| {
                run_with(day, |_, _| {});
            })
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
