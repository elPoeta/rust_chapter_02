use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let random = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut number = String::new();
        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed read input.");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&random) {
            Ordering::Less => println!("Try with BIG number!."),
            Ordering::Greater => println!("Try with SMALL number!."),
            Ordering::Equal => {
                println!("You Win!!! guess: {} -> secret: {}", number, random);
                break;
            }
        }
    }

    println!("Game over.");
}
