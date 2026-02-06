fn even_squares(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .copied()
        .filter(|value| value % 2 == 0)
        .map(|value| value * value)
        .collect()
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];
    println!("{:?}", even_squares(&values));
}

#[cfg(test)]
mod tests {
    use super::even_squares;

    #[test]
    fn builds_even_square_list() {
        assert_eq!(even_squares(&[1, 2, 3, 4]), vec![4, 16]);
    }
}
