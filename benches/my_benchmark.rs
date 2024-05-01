use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_julia::{imaginary::Imaginary, julia::calc_frame};

fn calc_dimension(dimension: u32) -> Vec<(u8, u8, u8)> {
    calc_frame(
        &(dimension, dimension),
        &4.0,
        &Imaginary { real: 0.0, i: 0.0 },
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dimension 500", |b| {
        b.iter(|| calc_dimension(black_box(500)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
