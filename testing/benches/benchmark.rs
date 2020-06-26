use base::parser::parse;
use base::renderer::Renderer;
use std::fs;
use ui::{Blocks, Inline, Wrapper};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark() {
    let json = fs::read_to_string("src.json").unwrap();
    let result = parse(json).unwrap();
    let renderer = Renderer::new(&result.record_map.block, Blocks {}, Inline {}, Wrapper {});
    let html = renderer
        .render("ddda599f-ff69-4974-9dec-86f6abf3209a")
        .to_string();

    fs::write("output.html", &html).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main", |b| b.iter(|| benchmark()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);