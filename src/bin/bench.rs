use sorting::*;

use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
struct SortEval<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEval<T> {
    fn eq(&self, other: &Self) -> bool {
        let cnt = self.cmps.get();
        self.cmps.set(cnt + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEval<T> {}

impl<T: PartialOrd> PartialOrd for SortEval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cnt = self.cmps.get();
        self.cmps.set(cnt + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEval<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let cnt = self.cmps.get();
        self.cmps.set(cnt + 1);
        self.t.cmp(&other.t)
    }
}

// measure the number of comparisons of a sorting algorithm
fn bench_cmps<T: Ord + Clone, S: Sorter>(sorter: S, values: &[T], counter: &Cell<usize>) -> usize {
    let mut values = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    count
}

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1000, 10000, 100000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEval {
                t: rng.gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }

        let took = bench_cmps(BubbleSorter, &values, &counter);
        println!("bubble {} {}", n, took);
        let took = bench_cmps(InsertionSorter { smart: true }, &values, &counter);
        println!("insertion_smart {} {}", n, took);
        let took = bench_cmps(InsertionSorter { smart: false }, &values, &counter);
        println!("insertion_unsmart {} {}", n, took);
        let took = bench_cmps(SelectionSorter, &values, &counter);
        println!("selection {} {}", n, took);
        let took = bench_cmps(QuickSorter, &values, &counter);
        println!("quick {} {}", n, took);
    }
}
