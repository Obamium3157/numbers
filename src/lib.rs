use std::io;
use std::cmp::Ordering;

pub enum Modes {
    NORMAL,
    LIMITED,
    VANGA,
}

impl Modes {
    pub fn new(mode: &str) -> Modes {
        match mode {
            "normal" => Modes::NORMAL,
            "limited" => Modes::LIMITED,
            "vanga" => Modes::VANGA,
            _ => panic!("??????????????")
        }
    }
}

pub fn select_mode(mode: Modes, secret_number: u32) {
    match mode {
        Modes::NORMAL => normal(secret_number),
        Modes::LIMITED => limited(secret_number),
        Modes::VANGA => vanga(secret_number),
    }
}

pub fn normal(secret_number: u32) {
    let mut attempts: u16 = 1;

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

pub fn limited(secret_number: u32) {
    let mut attempts: u16 = 1;

    loop {
        if attempts == 10 {
            println!("You lose!");
            println!("The answer was: {}", secret_number);
            break;
        }
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

pub fn vanga(secret_number: u32) {
    let mut attempts: u16 = 1;

    loop {
        if attempts == 2 {
            println!("You lose!");
            println!("The answer was: {}", secret_number);
            break;
        }
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
