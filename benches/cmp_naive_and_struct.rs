#![feature(box_syntax)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_cmp(c: &mut Criterion) {
    use adaptive_integrate::{integrate, Integrator};

    let functions: Vec<Box<dyn Fn(f64) -> f64>> = vec![box |_| 1., box |x| x * x, box |x| x.cos()];

    let mut group = c.benchmark_group("Integrate");
    for (i, f) in functions.into_iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Adaptive", i), &f, |b, f| {
            b.iter(|| Integrator::new(f).with_prec(0.001))
        });
        group.bench_with_input(BenchmarkId::new("Naive", i), &f, |b, f| {
            b.iter(|| integrate(100, f))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_cmp);
criterion_main!(benches);
