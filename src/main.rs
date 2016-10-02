extern crate rand; // use the dependency rand

use std::io; // allow access to stdin/stdout
// use the Trait Write of std::io to make it available in the scope
use std::io::Write;
// those 2 uses can be replaced with
// use std::io::{self, Write};

use std::cmp::Ordering; // import the enum in the scope

use rand::Rng; // import the Trait to the scope

fn main() {
    println!("----- Guess the number ! -----\n"); // println! auto flush the stream

    loop {
        // generate a random number in [1;101[
        let secret = rand::thread_rng().gen_range(1, 101);

        // for debug only
        // println!("The secret value is : {}", secret);

        loop {
            print!("Please input your guess: "); // print! need to be manually flushed
            io::stdout().flush().expect("Failed to flush stdout !");

            let mut guess = String::new(); // init a new mutable String
            io::stdin().read_line(&mut guess) // stdin() return a Stdin object
                .expect("Failed to read line !"); // read_line() return a Result object

            // shadow guess with immutable binding
            // parse() return a Result, and we will handle it to avoid crash
            let guess : u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_)  => continue,
            };

            println!("You guessed {}", guess);

            // compare the guessed value to the secret one
            match guess.cmp(&secret) {
                Ordering::Greater => println!("Too big !\n"),
                Ordering::Less    => println!("Too small !\n"),
                Ordering::Equal   => {
                    println!("You Win !\n");
                    break;
                },
            }
        }

        print!("Want to play again? [y/n]: ");
        io::stdout().flush().expect("Failed to flush stdout !");

        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line !");
        let again = String::from(again.trim());
        
        // use the "ref var foo if foo ..." to match Strings
        match again {
            ref x if x == "y" || x == "yes" => {
                println!("\n-----------------\n");
                continue
            },
            _ => break,
        }
    }
}
