fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9, 10];

    let total = sum(&v1) + sum(&v2);
    println!("Sum of all the vector elements: {}", total);
    println!("Multiplication of all the vector elements: {}", multiply(&v1));
}

fn sum(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn multiply(v: &Vec<i32>) -> i32 {
    return v.iter().fold(1, |a, &b| a * b);
}