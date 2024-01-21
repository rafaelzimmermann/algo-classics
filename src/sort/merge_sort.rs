use crate::sort::Sorter;

pub(crate) struct MergeSort;

impl MergeSort {
    fn mergesort(&self, slice: &mut [i32], helper: &mut[i32], low: &usize, high: &usize) {
        if *low < *high {
            let middle = low + (high - low) / 2;
            self.mergesort(slice, helper, low, &middle);
            self.mergesort(slice, helper, &(middle + 1), high);
            self.merge(slice, helper, low, &middle, high);
        }
    }

    fn merge(&self, slice: &mut [i32], helper: &mut [i32], low: &usize, middle: &usize, high: &usize) {
        for i in 0..slice.len() {
            helper[i] = slice[i];
        }

        let mut left: usize = *low;
        let mut right: usize = *middle + 1;
        let mut current: usize = *low;

        while left <= *middle && right <= *high {
            if helper[left] <= helper[right] {
                slice[current] = helper[left];
                left += 1;
            } else {
                slice[current] = helper[right];
                right += 1;
            }
            current += 1;
        }

        if *low != *middle {
            let remaining = (*middle - left) + 1;
            for _i in 0..remaining {
                slice[current] = helper[left];
                left += 1;
                current += 1;
            }
        } else {
            slice[current] = helper[left]
        }
    }

    fn init_sort(self, slice: &mut [i32]) {
        let mut help = Vec::with_capacity(slice.len());
        help.extend_from_slice(slice);
        self.mergesort(slice, &mut help, &0, &(slice.len() - 1))
    }
}

impl Sorter for MergeSort {
    fn new() -> MergeSort {
        MergeSort {}
    }
    

    fn sort(self, slice: &mut [i32]) {
        if slice.len() < 2 {
            return
        }
        self.init_sort(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_empty_slice() {
        let mut xs: [i32; 0] = [];
        let sorter: MergeSort = MergeSort::new();
        sorter.sort(&mut xs);
        assert!(xs.len() == 0, "Unexpected result");
    }

    #[test]
    fn test_sort_non_empty_slice() {
        let mut xs: [i32; 2] = [5, 4];
        let expected: [i32; 2] = [4, 5];
        let sorter: MergeSort = MergeSort::new();
        sorter.sort(&mut xs);
        assert!(
            xs.iter().zip(expected.iter()).all(|(a,b)| a == b),
            "Arrays are not equal"
        );
    }

    #[test]
    fn test_sort_non_even_slice() {
        let mut xs: [i32; 3] = [3, 5, 4];
        let expected: [i32; 3] = [3, 4, 5];
        let sorter: MergeSort = MergeSort::new();
        sorter.sort(&mut xs);
        assert!(
            xs.iter().zip(expected.iter()).all(|(a,b)| a == b),
            "Arrays are not equal"
        );
    }

    #[test]
    fn test_sort_non_even_slice_reversed() {
        let mut xs: [i32; 3] = [5, 4, 3];
        let expected: [i32; 3] = [3, 4, 5];
        let sorter: MergeSort = MergeSort::new();
        sorter.sort(&mut xs);
        assert!(
            xs.iter().zip(expected.iter()).all(|(a,b)| a == b),
            "Arrays are not equal"
        );
    }

    #[test]
    fn test_sort_slice_with_len_1() {
        let mut xs: [i32; 1] = [5];
        let expected: [i32; 1] = [5];
        let sorter: MergeSort = MergeSort::new();
        sorter.sort(&mut xs);
        assert!(xs.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
    }
}
