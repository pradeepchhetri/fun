fn main() {
    let mut v = vec![1,2,3,4,5,6,7];
    
    // Before pushing
    get_element(&v, 7);

    v.push(8);

    // After pushing    
    get_element(&v, 7);
}

fn get_element(v: &Vec<i32>, i: usize) {
    match v.get(i) {
        Some(x) => println!("Element at {} position: {}.", i, x),
        None    => println!("Sorry this vector is too short.")
    }
}