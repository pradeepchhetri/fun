use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed reading line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        if input == 42 {
            break;
        } else {
            println!("{}", input);
        }
    }    
}