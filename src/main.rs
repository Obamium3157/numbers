use std::{ io, env };
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let args: Vec<String> = env::args().collect();

    let max = &args[1].trim().parse().expect("Error");

    let secret_number = rand::thread_rng().gen_range(1..*max);
    let mut attempts :u32 = 1;

    loop {
        println!("\t\t\tAttempt №{}", attempts);
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        attempts += 1;
    }
}