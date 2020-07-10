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
            
            //for certain arrays, this is called when right is zero and
            //the application panics because usize cannot be negative
            //this wouldn't be a problem for a c style app using an int 
            //for range but given rust forces us to use usize we encounter
            //this problem
            if 0 < right {
                right -= 1;
            }
        }
    }

    left
}

fn swap_index(arr: &mut [i32], first: usize, second: usize) {
    let swap = arr[first];
    arr[first] = arr[second];
    arr[second] = swap;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn validate_array_is_sorted(arr: &mut [i32]) {
        let mut prev_val = 0;

        for i in 0..arr.len() {
            if 0 < i {
                if arr[i] < prev_val {
                    assert!(false, "array is not sorted!");
                    break;
                }
            }
            
            prev_val = arr[i];
        }
    }

    #[test]
    fn array_is_sorted() {
        let mut arr = [2, 6, 6, 5, 3, 7, 9, 1, 4, 9, 5];
        sort_int_array(&mut arr);

        validate_array_is_sorted(&mut arr);        
    }

    #[test]
    fn random_array_is_sorted() {
        let mut rng = rand::thread_rng();

        //let len: usize = rng.gen();
        //we cant dynamically set an array's length
        //arrays must have a defined length at compile time :()
        let mut arr = [0; 100];

        for i in 0..arr.len() {
            let val: i32 = rng.gen();
            arr[i] = val;
        }
        sort_int_array(&mut arr);

        validate_array_is_sorted(&mut arr);        
    }
}