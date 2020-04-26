use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gradient_lut::new_gradient::ColorGradient;

fn criterion_benchmark(c: &mut Criterion) {
    let mut gradient = ColorGradient::from_colors([255, 0, 0], [0, 0, 255]);
    gradient.add_color_at_t(0.5, [0, 255, 0]);

    c.bench_function("color_at_t (new f32)", |b| {
        b.iter(|| gradient.color_at_t(black_box(0.5)))
    });

    c.bench_function("color_at_t (new u8)", |b| {
        b.iter(|| gradient.color_at_t_u8(black_box(128)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
