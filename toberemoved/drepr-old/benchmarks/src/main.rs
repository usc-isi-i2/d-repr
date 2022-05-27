use benchmarks::readers::iterators::*;
use readers::prelude::{IndexIterator, Index};


fn main() {
    let n_steps = 6;
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
    
    println!("{:?}", results);
}