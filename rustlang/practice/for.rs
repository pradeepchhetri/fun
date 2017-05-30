fn main() {
    // Simple Counter
    for counter in 1..101 {
        println!("{}", counter);
    }

    // Enumeration
    for (index, counter) in (5..10).enumerate() {
        println!("Index: {} Value: {}", index, counter)
    }

    // Iterators
    let lines = "hello\nworld\nhow\nare\nyou".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("Num: {} Line: {}", linenumber, line)
    }
}