use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number game");

    let secret_number = rand::thread_rng().gen_range(1..=20);

    loop {
        let mut guess = String::new();
        
        println!("please input your number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                println!("the secret number is: {secret_number}");
                break;
            }
        }
    }

}
