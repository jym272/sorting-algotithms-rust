//Sort stable: equal values doesn't move
// mod bubblesort;

trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}




#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter{
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
}
    #[test]
    fn std_works() {
        let mut v = vec![2, 1, 3, 1];
        sort::<i32, StdSorter>(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3]);
    }
}
