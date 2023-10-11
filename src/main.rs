use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Welcome to the game guessing");

    let secret_number = rand::thread_rng().gen_range(1..=15);

    loop {
        println!("Please enter a number");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YOUR WIN !!!");
                break;
            }
        }
    }
}
