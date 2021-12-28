use std::env;
use rand::Rng;
use RustProject::{limited, Modes, normal, vanga};

fn main() {
    println!("Guess the number!");
    let args: Vec<String> = env::args().collect();

    let max = &args[1].trim().parse().expect("Error");

    let secret_number = rand::thread_rng().gen_range(1..*max);

    let mode = Modes::new(&args[2]);

    match mode {
        Modes::NORMAL => normal(secret_number),
        Modes::LIMITED => limited(secret_number),
        Modes::VANGA => vanga(secret_number),
    }
}