#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use canvas::tracer::Tracer;
use canvas::scene_loader;

fn trace_scene(n: u64) {
    let width = 400;
    let height = 400;
    let scene = scene_loader::load_scene("box").unwrap();
    let mut tracer = Tracer::new(
        scene,
        10,
        2.2,
        width,
        height,
    );

    let mut data = vec![0u8; width * height * 4];

    for _ in 0..n {
        tracer.update(&mut data);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trace scene", |b| b.iter(|| trace_scene(black_box(10000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
