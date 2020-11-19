// rust guessing game;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number");

    // createa a new random number from rand library
    let _secret_number = rand::thread_rng().gen_range(0, 101);

    println!("please input your number");
    // create new instance of a string that is mutable;
    let mut guess = String::new();

    // use the standard io libarary to read the input pro the user and save it in the staing "guess" which returns RESULT with a expect property to be called if the input could not be read;
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    let guess : i32 = guess.trim().parse()
        .expect("failed to parse, please use a valid number");

    // prints the input val that is saved in guess variable;
    println!("you guessed {}", guess);


    match guess.cmp(&_secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Equal => println!("Wiiner winner chicken dinner"),
        Ordering::Greater => println!("Too Big"),
    }
}
