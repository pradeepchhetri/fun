struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5; // same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("{}", f.x);
}