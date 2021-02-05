use super::Sorter;

pub struct BubbleSorter;

impl Sorter for BubbleSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                // If a previous one is greater than the latter one
                // swap them. So the order is ascending.
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
            // If swapped is false here,
            // it means no element is greater than the latter one
            // so the sorting is already complete
        }
    }
}

#[test]
fn bubble_works() {
    let mut things = vec![8, 2];
    BubbleSorter.sort(&mut things);
    assert_eq!(things, vec![2, 8]);
}
