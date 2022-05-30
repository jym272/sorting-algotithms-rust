use super::Sorter;

pub struct SelectionSort;
/// Implementing the Sorter trait for the SelectionSort struct.
impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted_index in 0..slice.len() {
            let min_index = slice[unsorted_index..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, value)| value)
                .map(|(index, _)| index + unsorted_index)
                .expect("empty slice");
            // 2do Option
            // let mut min_index = unsorted;
            // for j in unsorted..slice.len() {
            //     if slice[j] < slice[min_index] {
            //         min_index = j;
            //     }
            // }
            slice.swap(unsorted_index, min_index);
        }
    }
}
#[test]
fn selection_sort() {
    //vec of 42 items
    let mut v = vec![12, 11, 13, 5, 6, 7, 71, 8, 9, 10, 1, 2, 3, 4, 0, 54];
    SelectionSort.sort(&mut v);
    assert_eq!(
        v,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 54, 71]
    );
}
