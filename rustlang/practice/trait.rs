fn main() {
    let a = 10;
    let b = double(a);

    let c = false;
    let d = change_truth(c);

    println!("Value: {}", b);
    println!("Value: {}", d);
}

fn double(a: i32) -> i32 {
    return 2 * a
}

fn change_truth(c: bool) -> bool {
    return !c
}
