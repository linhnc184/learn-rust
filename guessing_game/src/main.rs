use rand::Rng;

use std::io;

use std::cmp::Ordering;

fn main() {
    println!("Guest the number game! You have 5 trials to guess a number in range 1 to 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess_times = 0;

    loop {
        if guess_times >=5 {
            println!("You lose!");
            return;
        }

        println!("Input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        guess_times += 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Too win!");
                break;
            },
        }
    }
}
