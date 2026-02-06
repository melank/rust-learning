fn calculate_length(text: &str) -> usize {
    text.len()
}

fn takes_and_returns(value: String) -> String {
    format!("{value}!")
}

fn main() {
    let message = String::from("hello");
    let length = calculate_length(&message);
    let next_message = takes_and_returns(message);

    println!("length: {length}");
    println!("next: {next_message}");
}

#[cfg(test)]
mod tests {
    use super::{calculate_length, takes_and_returns};

    #[test]
    fn borrows_without_moving() {
        let value = String::from("abc");
        let len = calculate_length(&value);
        assert_eq!(len, 3);
        assert_eq!(value, "abc");
    }

    #[test]
    fn moves_and_returns() {
        let value = String::from("rust");
        assert_eq!(takes_and_returns(value), "rust!");
    }
}
