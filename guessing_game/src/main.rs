use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing Game");

    let _secretkey = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter you guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line.");
        let guess: u32 = guess.trim().parse().expect("Please enter number.");
        match guess.cmp(&_secretkey) {
            Ordering::Less => println!("Guessed to low"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Guessed to high"),
        }
    }
}
