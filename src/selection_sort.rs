pub fn selection_sort<T, F>(arr: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i + 1..len {
            if compare(&arr[j], &arr[min_idx]) == Ordering::Less {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}