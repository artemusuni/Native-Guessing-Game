use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Number Guessing Game");
    let secret = rand::thread_rng().gen_range(1..=100);
    //println!("The number that was generated was: {secret}");
    loop {
        println!("Please Enter Your Guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        
        };
        println!("You have entered {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
    }



    }






}
