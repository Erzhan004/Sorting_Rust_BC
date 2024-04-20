pub fn quick_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr, compare);
    quick_sort(&mut arr[0..pivot_index], compare);
    quick_sort(&mut arr[pivot_index + 1..], compare);
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let pivot = arr[arr.len() / 2].clone();
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        while compare(&arr[i], &pivot) == Ordering::Less {
            i += 1;
        }
        while compare(&arr[j], &pivot) == Ordering::Greater {
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    }
    j
}