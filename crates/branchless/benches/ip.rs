use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let x = 10;
            for i in 0..1000 {
                let y = vec![1];
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
