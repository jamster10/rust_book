// use std::io;
// use std::cmp::Ordering
use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("please enter a guess.");
        let sec_num = thread_rng().gen_range(1..=5);
        io::stdin().read_line(&mut guess).expect("you fucked up");

        println!("secret num is {sec_num}");
        println!("You guessed {guess}");
        let spaces = "".len();
        let a = if true { true } else { "six" };

        let guess: u32 = guess.trim().parse().expect("Yo give me a number");

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too Low"),
            Ordering::Equal => println!("You nailed it!"),
            Ordering::Greater => println!("Too high"),
        }
    }
}
