fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original_string = "Hello, world!";
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}


Output
Original string: Hello, world!
Reversed string: !dlrow ,olleH

[Execution complete with exit code 0]