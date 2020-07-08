pub fn sort_int_array(arr: &mut [i32]) {
    let mut copy_of_arr: Vec<i32> = vec![0; arr.len()];

    heap_sort_range(arr, &mut copy_of_arr, 0, arr.len() - 1);
}

fn heap_sort_range(arr: &mut [i32], copy_of_arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high
    {
        let mid_index = low + (high - low)/2;
        
        heap_sort_range(arr, copy_of_arr, low, mid_index);
        heap_sort_range(arr, copy_of_arr, mid_index + 1, high);

        merge_sorted_arrays(arr, copy_of_arr, low, mid_index, high);
    }
}

fn merge_sorted_arrays(arr: &mut [i32], copy_of_arr: &mut Vec<i32>, low: usize, mid: usize, high: usize) {
    for i in low..=high {
        copy_of_arr[i] = arr[i];
    }
        
    let mut left: usize = low;
    let mut right: usize = mid + 1;
    let mut counter: usize = low;

    while counter <= high {
        while left <= mid && right <= high {
            if copy_of_arr[left] <= copy_of_arr[right] {
                arr[counter] = copy_of_arr[left];
                left += 1;
            }
            else {
                arr[counter] = copy_of_arr[right];
                right += 1;
            }

            counter += 1;
        }

        if left <= mid {
            arr[counter] = copy_of_arr[left];
            left += 1;
        }

        //the right half is already copied and can be skipped
       
        counter += 1;
    }
}