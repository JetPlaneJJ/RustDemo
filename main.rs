// Notes from Microsoft Build 2020 May 19th
// rust is more secure than C++, better understanding if programs correct
// more memory saving than C++
use std::io;
use rand::Rng;
use std::cmp::Ordering; //enum Ordering, like compareTo in Java

fn main() {
    let num = rand::thread_rng().gen_range(1,11);
    // why the "!" ? It is to denote a macro, NOT a function!
    // Doc: "Fundamentally, macros are a way of writing code that writes other 
    // code, which is known as metaprogramming." Macros executed @ runtime
    // Functions are --> 
    println!("Guess a number between 1 and 10: ");
    loop {
        // mut = mutable variable (changable), Rust by default does immutable
        // Create a new String object
        let mut guess = String::new();
        
        // take in input
        io::stdin()
            .read_line(&mut guess) // creating reference to var w/out copy 2 mem
            .expect("Failed to read line");
        
        // {} placeholder for input
        println!("You guessed {}", guess);
        // Note for running: cargo run --> run app
        
        // change guess unsigned 32 bit integer 
        // trim off any whitespace with parse
        let guess: u32 = guess.trim().parse().expect("Please type a number");
        
        // Shadowing --> two vars with same name allowed
        // Wikipedia: ariable declared within a certain scope 
        // (decision block, method, or inner class) 
        // has the same name as a variable declared in an outer scope. 
        // At the level of identifiers (names, rather than variables), 
        // this is known as name masking. 
        // This outer variable is said to be shadowed by the inner variable, 
        // while the inner identifier is said to mask the outer identifier
        
        // Like Switch statements in Java
        match guess.cmp(&num) {
            Ordering::Less => println!("Too low..."),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}