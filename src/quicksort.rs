use super::Sorter;
use std::fmt::Display;
pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T: Display>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let len = slice.len();
        match len {
            0 | 1 => (),
            2 => {
                if slice[0] > slice[1] {
                    slice.swap(0, 1);
                }
            }
            _ => {
                let pivot = len - 1;
                let mut i = 0;
                let mut j = len - 2;
                while i < j {
                    // dbg!(i, j);
                    //slice to string transfor
                    let _ = slice.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                    // dbg!(slice_to_string);
                    if slice[i] > slice[pivot] && slice[j] < slice[pivot] {
                        slice.swap(i, j);
                    } else {
                        if slice[i] <= slice[pivot] {
                            i += 1;
                        }
                        if slice[j] >= slice[pivot] {
                            j -= 1;
                        }
                    }
                }

                let mut swap = 1;

                if slice[i] > slice[pivot] {
                    slice.swap(i, pivot);
                    swap = 0;
                } else {
                    slice.swap(i + 1, pivot);
                    swap = 1;
                }

                let delimeter = i + swap;

                self.sort(&mut slice[0..delimeter]);
                self.sort(&mut slice[delimeter + 1..]);
            }
        }
    }
}
#[test]
fn quick_sort() {
    let mut v = vec![
        12, 11, 2, 11, 1, 15, 14, 86, 74, 36, 57, 14, 2, 21, 47, 89, 56, 32, 3, 4, 52, -2, 0, 54,
        -5,
    ];

    QuickSort.sort(&mut v);
    assert_eq!(
        v,
        vec![
            -5, -2, 0, 1, 2, 2, 3, 4, 11, 11, 12, 14, 14, 15, 21, 32, 36, 47, 52, 54, 56, 57, 74,
            86, 89
        ]
    );
}
