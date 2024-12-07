use advent_of_code::FUNCS;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    for (day, (f1, f2)) in FUNCS.iter().enumerate().map(|(i, f)| (i + 1, f)) {
        if let Ok(input) = std::fs::read_to_string(format!("inputs/{day}.txt")) {
            c.bench_function(&format!("day_{day}_1"), |b| {
                b.iter(|| {
                    f1(&input);
                });
            });
            c.bench_function(&format!("day_{day}_2"), |b| {
                b.iter(|| {
                    f2(&input);
                });
            });
        }
    }
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
