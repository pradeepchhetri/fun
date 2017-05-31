fn main() {
    'outer: for i in 1..10 {
        'inner: for j in 1..10 {
            if i % 2 == 0 { continue 'outer; } // Continues the outer loop
            if j % 2 == 0 { continue 'inner; } // Continues the inner loop
            println!{"i: {} & j: {}", i, j};
        }
    }
}