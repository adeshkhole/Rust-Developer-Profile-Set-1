fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target && (mid == 0 || arr[mid - 1] < target) {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let arr = vec![1, 2, 2, 2, 3, 4, 5, 6];
    let target = 2;

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} is not present in the array.", target),
    }
}


Output
First occurrence of 2 is at index 1

[Execution complete with exit code 0]