use std::env;
use rand::Rng;
use numbers::{ Modes, select_mode };

fn main() {
    println!("Guess the number!");
    let args: Vec<String> = env::args().collect();

    let max = &args[1].trim().parse().expect("Error");

    let secret_number = rand::thread_rng().gen_range(1..*max);

    let mode = Modes::new(&args[2]);

    select_mode(mode, secret_number);
}
