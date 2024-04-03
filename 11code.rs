fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    
    result
}

fn main() {
    let arr1 = [1, 3, 5, 7, 9];
    let arr2 = [2, 4, 6, 8, 10];
    
    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    
    println!("Merged sorted array: {:?}", merged_array);
}


Output
Merged sorted array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

[Execution complete with exit code 0]