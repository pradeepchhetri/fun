fn main() {
    let mut x = 10;
    {
        let y = &mut x;
        *y += 1;
    }
    println!{"x: {}", x};
}