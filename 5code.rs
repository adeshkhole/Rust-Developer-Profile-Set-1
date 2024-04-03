fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // If the length of the array is even, return the average of the middle two elements
        let mid_left = arr[len / 2 - 1];
        let mid_right = arr[len / 2];
        return (mid_left as f64 + mid_right as f64) / 2.0;
    } else {
        // If the length of the array is odd, return the middle element
        return arr[len / 2] as f64;
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];
    
    println!("Median of arr1: {}", find_median(&arr1)); // Output: 3
    println!("Median of arr2: {}", find_median(&arr2)); // Output: 3.5
}



Output
Median of arr1: 3
Median of arr2: 3.5

[Execution complete with exit code 0]