use std::io;

fn main() {
    println!("Guessing the number!");

    println!("Please input your number.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess {0}", guess);
}
