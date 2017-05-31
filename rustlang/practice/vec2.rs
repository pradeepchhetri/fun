fn main() {
    let mut v = vec![1,2,3,4,5,6,7];
    
    // Before pushing
    get_element(&v, 7);

    // Print all elements
    print_all(&v);

    v.push(8);

    // After pushing    
    get_element(&v, 7);

    // Print all elements
    print_all(&v);
}

fn get_element(v: &Vec<i32>, i: usize) {
    match v.get(i) {
        Some(x) => println!("Element at {} position: {}.", i, x),
        None    => println!("Sorry this vector is too short.")
    }
}

fn print_all(v: &Vec<i32>) {
    println!("Printing all the elements of vector:");
    for i in v {
        println!("{}", i);
    }
}