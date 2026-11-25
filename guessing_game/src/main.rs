use std::io;
use rand::Rng;

//use std::cmp::Ordering;

//rust inbuild functionality to generate random numbers is not available in rust standard library, so we need to add the rand crate to our project. To do this, we need to add the following line to our Cargo.toml file: 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    loop{
        println!("Please input your guess.");

        let mut guess : String = String::new();

        //mutable variable guess is created to store the user input also reference is also mutable accepted
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); // if the msg is error the execpt will be called and the program will terminate with the error message

        //Shadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable. The new variable shadows the previous variable, meaning that the previous variable is no longer accessible. This is useful for converting a variable from one type to another, as we are doing here with the guess variable.
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); //trim() removes any whitespace from the input and parse() converts the string to a number
        
        println!("You guessed: {guess}"); 

        //match is like switch statement in other languages, it allows you to compare a value against a series of patterns and execute code based on which pattern matches. In this case, we are comparing the guess variable against the secret_number variable and printing a message based on whether the guess is too low, too high, or correct.
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too large!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;  
        }
    }   

} 
