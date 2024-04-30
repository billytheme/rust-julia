use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_julia::{imaginary::Imaginary, julia::calc_pixel};

fn calc_dimension(dimension: u32) -> Vec<(u8, u8, u8)> {
    let mut res = vec![];
    for x in 0..dimension {
        for y in 0..dimension {
            res.push(calc_pixel(
                (x, y),
                (dimension, dimension),
                4.0,
                Imaginary { real: 0.0, i: 0.0 },
            ))
        }
    }
    res
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dimension 500", |b| {
        b.iter(|| calc_dimension(black_box(500)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
