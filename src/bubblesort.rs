use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let mut swapped = false;
            for j in 0..slice.len() - i - 1 {
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }
}

#[test]
fn bubble_sort() {
    //vec of 42 items
    let mut v = vec![12, 11, 13, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 0, 54];
    BubbleSort.sort(&mut v);
    assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 54]);
}
