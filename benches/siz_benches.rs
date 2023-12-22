use criterion::{criterion_group, criterion_main, Criterion};

use siz::fuzzy::levenshtein_distance;

fn bench_levenshtein_distance(c: &mut Criterion) {
    c.bench_function("levenshtein_distance", |b| {
        b.iter(|| levenshtein_distance("sitting", "kitten"))
    });
}

criterion_group!(benches, bench_levenshtein_distance);
criterion_main!(benches);
