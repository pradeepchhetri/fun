fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    for i in &v {
        println!("{}", i);
    }
}