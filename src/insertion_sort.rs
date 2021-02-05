use super::*;

pub struct InsertionSorter {
    pub smart: bool,
}

impl Sorter for InsertionSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | unsorted ]
        // `unsorted` : the starting index fo the unsorted part
        // the first element of the slice is always considered as sorted.
        let len = slice.len();
        for unsorted in 1..len {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // search the right location in the sorted part of the slice
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                // X O O O Y
                // Y X O O O
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn unsmart_insertion_works() {
    let mut things = vec![8, 2, 1, 3, 5];
    let insertion_sorter = InsertionSorter { smart: false };
    insertion_sorter.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 5, 8]);
}

#[test]
fn smart_insertion_works() {
    let mut things = vec![8, 2, 1, 3, 5];
    let insertion_sorter = InsertionSorter { smart: true };
    insertion_sorter.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 5, 8]);
}
