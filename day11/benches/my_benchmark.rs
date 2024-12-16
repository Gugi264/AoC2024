use std::string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day11::{count_digits, split_in_two};

pub fn criterion_benchmark(c: &mut Criterion) {
    let nr = 1234345235;
    c.bench_function("count digits division", |b| b.iter(|| count_digits(nr)));
    c.bench_function("count digits string", |b| b.iter(|| string_count(nr)));
    c.bench_function("split in two", |b| b.iter(|| split_in_two(nr)));
}

fn string_count(nr: usize) -> usize {
    nr.to_string().len()
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
