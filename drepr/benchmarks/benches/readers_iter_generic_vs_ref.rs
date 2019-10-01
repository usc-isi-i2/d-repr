#[macro_use]
extern crate criterion;

use benchmarks::readers::iterators::*;
use criterion::{black_box, Criterion, ParameterizedBenchmark};
use readers::prelude::{Index, IndexIterator};

fn iter_range_ref(n_steps: usize) -> usize {
    let mut idx = vec![Index::Idx(0); n_steps];
    let unfrozen_dims = (0..n_steps).rev().collect::<Vec<_>>();
    let lowerbounds = vec![0; n_steps];
    let upperbounds = vec![4; n_steps];
    let steps = vec![1; n_steps];

    let mut iter = KnownRangeIterOld::new(&mut idx, unfrozen_dims, lowerbounds, upperbounds, steps);
    let mut results = 0;
    loop {
        results += iter.value().len();
        if !iter.advance() {
            break;
        }
    }

    return results;
}

fn iter_range_generic_vec(n_steps: usize) -> usize {
    let idx = vec![Index::Idx(0); n_steps];
    let unfrozen_dims = (0..n_steps).rev().collect::<Vec<_>>();
    let lowerbounds = vec![0; n_steps];
    let upperbounds = vec![4; n_steps];
    let steps = vec![1; n_steps];

    let mut iter: KnownRangeIter<Vec<Index>> =
        KnownRangeIter::new(idx, unfrozen_dims, lowerbounds, upperbounds, steps);
    let mut results = 0;
    loop {
        results += iter.value().len();
        if !iter.advance() {
            break;
        }
    }

    return results;
}

fn iter_range_generic_ref(n_steps: usize) -> usize {
    let mut idx = vec![Index::Idx(0); n_steps];
    let unfrozen_dims = (0..n_steps).rev().collect::<Vec<_>>();
    let lowerbounds = vec![0; n_steps];
    let upperbounds = vec![4; n_steps];
    let steps = vec![1; n_steps];

    let mut iter: KnownRangeIter<&mut [Index]> =
        KnownRangeIter::new(&mut idx, unfrozen_dims, lowerbounds, upperbounds, steps);
    let mut results = 0;
    loop {
        results += iter.value().len();
        if !iter.advance() {
            break;
        }
    }

    return results;
}

fn bench_fibs(c: &mut Criterion) {
    c.bench(
        "range_iter_generic_vs_ref",
        ParameterizedBenchmark::new(
            "generic_vec",
            |b, n_steps| b.iter(|| iter_range_generic_vec(black_box(*n_steps))),
            vec![6, 7],
        )
        .with_function("ref", |b, n_steps| {
            b.iter(|| iter_range_ref(black_box(*n_steps)))
        })
        .with_function("generic_ref", |b, n_steps| {
            b.iter(|| iter_range_generic_ref(black_box(*n_steps)))
        }),
    );
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
