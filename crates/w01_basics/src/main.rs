fn increment(value: i32) -> i32 {
    value + 1
}

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = increment(x);
    println!("The value of x is {x}");
}

#[cfg(test)]
mod tests {
    use super::increment;

    #[test]
    fn increment_adds_one() {
        assert_eq!(increment(5), 6);
    }
}
