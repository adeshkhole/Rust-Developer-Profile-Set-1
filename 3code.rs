fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    
    match shortest_word(sentence) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the string."),
    }
}


Output
Shortest word: The

[Execution complete with exit code 0]