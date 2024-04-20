pub fn merge_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left, compare);
    merge_sort(&mut right, compare);
    merge(&left, &right, arr, compare);
}

fn merge<T, F>(left: &[T], right: &[T], arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let (mut left_idx, mut right_idx, mut arr_idx) = (0, 0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if compare(&left[left_idx], &right[right_idx]) != Ordering::Greater {
            arr[arr_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            arr[arr_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        arr_idx += 1;
    }

    while left_idx < left.len() {
        arr[arr_idx] = left[left_idx].clone();
        left_idx += 1;
        arr_idx += 1;
    }

    while right_idx < right.len() {
        arr[arr_idx] = right[right_idx].clone();
        right_idx += 1;
        arr_idx += 1;
    }
}