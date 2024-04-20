//lib.rs 
pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = arr.to_vec();
    let (mut i, mut j, mut k) = (0, mid, 0);
    while i < mid && j < len {
        if arr[i] <= arr[j] {
            merged[k] = arr[i].clone();
            i += 1;
        } else {
            merged[k] = arr[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        merged[k] = arr[i].clone();
        i += 1;
        k += 1;
    }

    while j < len {
        merged[k] = arr[j].clone();
        j += 1;
        k += 1;
    }

    arr.copy_from_slice(&merged);
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_ind = i;
        for j in i + 1..len {
            if arr[j] < arr[min_ind] {
                min_ind = j;
            }
        }
        if i != min_ind {
            arr.swap(i, min_ind);
        }
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_ind = partition(arr);
    quick_sort(&mut arr[..pivot_ind]);
    quick_sort(&mut arr[pivot_ind + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_ind = len / 2;
    arr.swap(pivot_ind, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}
