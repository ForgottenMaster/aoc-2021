use {
    aoc_2021::run,
    criterion::{criterion_group, criterion_main, Criterion},
};

fn criterion_benchmark(c: &mut Criterion) {
    (1..=7).for_each(|day| {
        c.bench_function(&format!("run_day_{}", day), |b| {
            b.iter(|| {
                run(day).unwrap();
            })
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
