use std::io;

fn main() {
    let mut done = false;
    println!("Guess a number.");

    while !done {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read !!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        if guess == 42 {
            println!("Hurray. You guessed it right.");
            done = true;
        } else {
            println!("Sorry wrong guess. Try again.");
        }
    }
}