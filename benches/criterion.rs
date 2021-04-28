use criterion::*;

fn criterion(c: &mut criterion::Criterion) {

    let mut group = c.benchmark_group("vec");

    let mut vec = Vec::new();
    group.bench_function("push", |b| b.iter(|| {
        for _ in 0..1000 {
            vec.push(black_box(0));
        }
    }));

    let mut vec = Vec::new();
    group.bench_function("insert", |b| b.iter(|| {
        for _ in 0..1000 {
            vec.insert(vec.len(), black_box(0));
        }
    }));

    group.finish();
}

criterion_group!(benches, criterion);
criterion_main!(benches);
