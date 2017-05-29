fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("CracklePop");
        } else if n % 3 == 0 {
            println!("Crackle");
        } else if n % 5 == 0 {
            println!("Pop");
        } else {
            println!("{}", n);
        }
    }
}
