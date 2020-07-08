mod quick_sort_int_array;
mod heap_sort;

fn main() {
    println!("Hello, quick sort!");

    let mut arr = [2, 8, 6, 5, 3, 7, 9, 1, 4];
    println!("before sorting: {:?}", arr);   

    quick_sort_int_array::sort_int_array(&mut arr);    
    println!("after quick sorting: {:?}", arr);   

    println!("Hello, heap sort!");

    let mut arr = [2, 8, 6, 5, 3, 7, 9, 1, 4];
    println!("before sorting: {:?}", arr);   

    heap_sort::sort_int_array(&mut arr);    
    println!("after heap sorting: {:?}", arr);   
}   