pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubble_sort;
mod insertion_sort;
mod quick_sort;
mod selection_sort;

pub use bubble_sort::BubbleSorter;
pub use insertion_sort::InsertionSorter;
pub use quick_sort::QuickSorter;
pub use selection_sort::SelectionSorter;

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![1, 4, 3, 2];
        StdSorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4])
    }
}
