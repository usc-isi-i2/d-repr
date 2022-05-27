#[macro_use]
extern crate criterion;

use benchmarks::readers::iterators::*;
use criterion::{black_box, Criterion, ParameterizedBenchmark};
use readers::prelude::{Index, IndexIterator};

fn roll_fn(size: usize, m: usize) -> usize {
  let mut total = 0;
  for i in 0..size {
    let k = i % 5;
    for j in 0..m {
      total += j * k + (j * j) * k;
    }
    
//    for j in 4..8 {
//      total += j * k;
//    }
  }
  return total;
}

fn unroll_fn(size: usize) -> usize {
  let mut total = 0;
  for i in 0..size {
    let k = i % 5;
    total += k * 0 + (0 * 0) * k;
    total += k * 1 + (1 * 1) * k;
    total += k * 2 + (2 * 2) * k;
    total += k * 3 + (3 * 3) * k;
//
//    total += k * 4;
//    total += k * 5;
//    total += k * 6;
//    total += k * 7;
//    total += k * 8;
//    total += k * 9;
//    total += k * 10;
//    total += k * 11;
//    total += k * 12;
//    total += k * 13;
//    total += k * 14;
//    total += k * 15;
//    total += k * 16;
//    total += k * 17;
//    total += k * 18;
//    total += k * 19;
  }
  return total;
}

fn bench_fn(c: &mut Criterion) {
  c.bench(
    "roll_vs_unroll",
    ParameterizedBenchmark::new(
      "roll",
      |b, size| b.iter(|| roll_fn(black_box(*size), black_box(4))),
      vec![25000, 50000]
    )
    .with_function("unroll", |b, size| b.iter(|| unroll_fn(black_box(*size))))
  );
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);