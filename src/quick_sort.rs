use super::Sorter;

pub struct QuickSorter;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            } else {
                return;
            }
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            // move the element to the right part if the value of the element is larger than the
            // pivot
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    // pivot <-> the last element of the left part
    slice.swap(0, left);

    let (smaller, bigger) = slice.split_at_mut(left + 1);
    quicksort(smaller);
    quicksort(bigger);
}

impl Sorter for QuickSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
    }
}

#[test]
fn quick_works() {
    let mut things = vec![7, 8, 2, 1, 3, 5, 12];
    QuickSorter.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 5, 7, 8, 12]);
}
