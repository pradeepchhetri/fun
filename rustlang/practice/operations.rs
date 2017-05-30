fn main() {
    print_sum(1, 10);
    print_difference(1, 10);
    print_multiplication(1, 10);
    print_division(1, 10);
}

fn print_sum(x: i32, y: i32) {
    println!("{}", x + y);
}

fn print_difference(x: i32, y: i32) {
    println!("{}", x - y);
}

fn print_multiplication(x: i32, y: i32) {
    println!("{}", x * y);
}

fn print_division(x: i32, y: i32) {
    println!("{}", x / y);
}