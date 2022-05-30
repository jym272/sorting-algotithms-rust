//Sort stable: equal values doesn't move
mod bubblesort;
mod insertionsort;
mod selectionsort;
/// A trait that defines a function called sort that takes a mutable slice of T where T is Ord.
trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

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
        let mut v = vec![2, 1, 3, 1];
        StdSorter.sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3]);
    }
}
