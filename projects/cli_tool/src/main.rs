use std::cmp::Ordering;
use std::io;

fn next_secret(current: u32) -> u32 {
    let next = current.saturating_mul(37).saturating_add(11);
    (next % 100).max(1)
}

fn main() {
    println!("Guess the number!");

    let secret_number = 42;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                let next = next_secret(secret_number);
                println!("Next round secret preview: {next}");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::next_secret;

    #[test]
    fn secret_number_stays_in_range() {
        let value = next_secret(42);
        assert!((1..=99).contains(&value));
    }
}
