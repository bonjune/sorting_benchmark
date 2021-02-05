use super::*;

pub struct SelectionSorter;

impl Sorter for SelectionSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | unsorted ]
        let len = slice.len();
        for unsorted in 0..len {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| i + unsorted)
                .expect("slice is non-empty");

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut things = vec![8, 2, 1, 3, 5];
    SelectionSorter.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 5, 8]);
}
