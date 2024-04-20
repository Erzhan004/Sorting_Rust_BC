pub fn insertion_sort<T, F>(arr: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) == Ordering::Less {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}