fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let string1 = "racecar";
    let string2 = "Hello World";

    println!("{}", is_palindrome(string1)); // Output: true
    println!("{}", is_palindrome(string2)); // Output: false
}


Output
true
false

[Execution complete with exit code 0]