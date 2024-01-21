mod bubble_sort;
mod merge_sort;


trait Sorter {
    fn new() -> Self;

    fn sort(self, slice: &mut [i32]);
}