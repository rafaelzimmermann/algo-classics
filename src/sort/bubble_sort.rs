use crate::sort::Sorter;

pub(crate) struct BubbleSort;

impl BubbleSort {
    fn swap(slice: &mut [i32], i: usize, j:usize) {
        slice.swap(i, j);
    }
}

impl Sorter for BubbleSort {
    fn new() -> BubbleSort {
        BubbleSort {}
    }
    

    fn sort(self, slice: &mut [i32]) {
        if slice.len() < 2 {
            return
        }
        for i in 0..slice.len()-1 {
            for j in i+1..slice.len() {
                if slice.get(i) > slice.get(j) {
                    BubbleSort::swap(slice, i, j)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_empty_slice() {
        let mut xs: [i32; 0] = [];
        let sorter: BubbleSort = BubbleSort::new();
        sorter.sort(&mut xs);
        assert!(xs.len() == 0, "Unexpected result");
    }

    #[test]
    fn test_sort_non_empty_slice() {
        let mut xs: [i32; 2] = [5, 4];
        let expected: [i32; 2] = [4, 5];
        let sorter: BubbleSort = BubbleSort::new();
        sorter.sort(&mut xs);
        assert!(xs.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }

    #[test]
    fn test_sort_slice_with_len_1() {
        let mut xs: [i32; 1] = [5];
        let expected: [i32; 1] = [5];
        let sorter: BubbleSort = BubbleSort::new();
        sorter.sort(&mut xs);
        assert!(xs.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}

 


