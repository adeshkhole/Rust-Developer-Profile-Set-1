fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    
    if n <= 3 {
        return true; // 2 and 3 are prime numbers
    }
    
    if n % 2 == 0 || n % 3 == 0 {
        return false; // Numbers divisible by 2 or 3 (except 2 and 3 themselves) are not prime
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false; // If n is divisible by any number from 5 to sqrt(n), it's not prime
        }
        i += 6;
    }
    
    true // If none of the above conditions are met, n is prime
}

fn main() {
    let num = 17;
    
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}



Output
17 is a prime number.

[Execution complete with exit code 0]