fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // Return None if k is greater than the length of the array
    }
    
    let mut sorted_arr = arr.to_vec(); // Create a mutable copy of the array
    sorted_arr.sort(); // Sort the array in ascending order
    
    Some(sorted_arr[k - 1]) // Return the kth smallest element (0-indexed)
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is {}", k, smallest),
        None => println!("Invalid input: k is greater than the length of the array"),
    }
}


Output
The 3th smallest element is 7

[Execution complete with exit code 0]