use std::num::ParseIntError;

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    input.parse::<u16>()
}

fn main() {
    match parse_port("8080") {
        Ok(port) => println!("port: {port}"),
        Err(err) => eprintln!("parse error: {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn parses_valid_port() {
        assert_eq!(parse_port("3000").ok(), Some(3000));
    }

    #[test]
    fn fails_on_invalid_number() {
        assert!(parse_port("abc").is_err());
    }
}
