// rust guessing game;
use std::io;

fn main() {

    println!("Guess the number");
    println!("please input your number");

    // create new instance of a string that is mutable;
    let mut guess = String::new();


    // use the standard io libarary to read the input pro the user and save it in the staing "guess" which returns RESULT with a expect property to be called if the input could not be read;
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    // prints the input val that is saved in guess variable;
    println!("you guessed {}", guess);
}
