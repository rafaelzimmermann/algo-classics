mod bubble_sort;


trait Sorter {
    fn new() -> Self;

    fn sort(self, slice: &mut [i32]);
}