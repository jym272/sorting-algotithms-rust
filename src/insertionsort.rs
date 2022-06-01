use super::Sorter;

pub struct InsertionSort {
    pub with_binary_search: bool,
}

/// Implementing the trait Sorter for the struct InsertionSort.
impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let binary_search = self.with_binary_search; //complexity O(log(n)*n)
        for unsorted_position in 1..slice.len() {
            if !binary_search {
                let mut j = unsorted_position;
                while j > 0 && slice[j] < slice[j - 1] {
                    slice.swap(j, j - 1);
                    j -= 1;
                }
            } else {
                let new_position =
                    match slice[..unsorted_position].binary_search(&slice[unsorted_position]) {
                        Ok(j) => j,  //found
                        Err(j) => j, //j is the index where the value should be inserted
                    };
                slice[new_position..=unsorted_position].rotate_right(1);
            }
        }
    }
}
#[test]
fn insertion_sort() {
    //vec of 42 items
    let mut v = vec![12, 11, 13, 5, 6, 7, 71, 8, 9, 10, 1, 2, 3, 4, 0, 54];
    InsertionSort {
        with_binary_search: false,
    }
    .sort(&mut v);
    assert_eq!(
        v,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 54, 71]
    );
}

#[test]
fn insertion_sort_smart() {
    //vec of 42 items
    let mut v = vec![
        12, 11, 2, 11, 1, 15, 14, 86, 74, 36, 57, 14, 2, 21, 47, 89, 56, 32, 3, 4, 52, -2, 0, 54,
        -5,
    ];
    InsertionSort {
        with_binary_search: true,
    }
    .sort(&mut v);
    assert_eq!(
        v,
        vec![
            -5, -2, 0, 1, 2, 2, 3, 4, 11, 11, 12, 14, 14, 15, 21, 32, 36, 47, 52, 54, 56, 57, 74,
            86, 89
        ]
    );
}
