//how many comparissons

use rand::prelude::SliceRandom;
use rand::Rng;
use sorting_algoritms::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    compares: Rc<Cell<usize>>,
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: Eq> PartialEq<Self> for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.compares.set(self.compares.get() + 1);
        self.t == other.t
    }
}

impl<T: Ord> PartialOrd<Self> for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.compares.set(self.compares.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compares.set(self.compares.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    //making sure the values are sorted
    let count = counter.get();
    assert!(values.windows(2).all(|w| w[0] <= w[1]));
    (count, time.elapsed().as_secs_f64())
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                compares: Rc::clone(&counter),
            });
        }
        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("bubble \t\t{}\t {}\t {}", n, took.0, took.1);
            let took = bench(
                InsertionSort {
                    with_binary_search: true,
                },
                &values,
                &counter,
            );
            println!("insertionBS \t{}\t {} \t {}", n, took.0, took.1);
            let took = bench(
                InsertionSort {
                    with_binary_search: false,
                },
                &values,
                &counter,
            );
            println!("insertion \t{}\t {} \t {}", n, took.0, took.1);

            let took = bench(SelectionSort, &values, &counter);
            println!("selection \t{}\t {} \t {}", n, took.0, took.1);

            let took = bench(QuickSort, &values, &counter);
            println!("quick \t\t{}\t {} \t {}", n, took.0, took.1);

            let took = bench(StdSorter, &values, &counter);
            println!("std \t\t{}\t {} \t {}", n, took.0, took.1);
        }
    }
}
