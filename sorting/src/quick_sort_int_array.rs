//==== using array - i32 only ====

pub fn sort_int_array(arr: &mut [i32]) {
    quick_sort_range(arr, 0, arr.len() - 1);
}

fn quick_sort_range(arr: &mut[i32], low: usize, high: usize) {

    if low < high {
        let index = partition(arr, low, high);

        if low < index - 1{
            quick_sort_range(arr, low, index - 1);
        }
        if index < high {
            quick_sort_range(arr, index, high);
        }
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = low + (high - low)/2;
    let pivot_value = arr[pivot];

    let mut left: usize = low;
    let mut right: usize = high;

    while left <= right {
        while arr[left] < pivot_value {
            left += 1;
        }
        while arr[right] > pivot_value {
            right -= 1;
        }

        if left <= right {
            swap_index(arr, left, right);
            left += 1;
            right -= 1;
        }
    }

    left
}

fn swap_index(arr: &mut [i32], first: usize, second: usize) {
    let swap = arr[first];
    arr[first] = arr[second];
    arr[second] = swap;
}