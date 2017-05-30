fn main() {
    for i in 1..1000 {
        if i % 999 == 0 {
            return;
        } else if i % 2 == 0 {
            continue;
        } else if i % 3 == 0 {
            println!("Crackle");
        } else if i % 5 == 0 {
            println!("Pop");
        } else {
            println!("{}", i);
        }
    }
}