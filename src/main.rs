use std::fs::File;
use std::io::{Read, Error};

mod lex;
mod data;
mod parse;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn parse_file(filename: &str) -> Result<(), Error> {
    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut tokens = lex::lex(&mut s.as_str())?;
    let json_data = parse::parse(&mut tokens)?;
    println!("{:?}", json_data);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_1_valid() {
        assert!(parse_file("tests/step1/valid.json").is_ok());
    }

    #[test]
    fn test_step_1_invalid() {
        assert!(parse_file("tests/step1/invalid.json").is_err());
    }

    #[test]
    fn test_step_2_valid() {
        assert!(parse_file("tests/step2/valid.json").is_ok());
    }

    #[test]
    fn test_step_2_invalid() {
        assert!(parse_file("tests/step2/invalid.json").is_err());
    }

    #[test]
    fn test_step_2_valid_2() {
        println!("{:?}", parse_file("tests/step2/valid2.json"));
        assert!(parse_file("tests/step2/valid2.json").is_ok());
    }

    #[test]
    fn test_step_2_invalid_2() {
        assert!(parse_file("tests/step2/invalid2.json").is_err());
    }

    #[test]
    fn test_step_3_valid() {
        assert!(parse_file("tests/step3/valid.json").is_ok());
    }

    #[test]
    fn test_step_3_invalid() {
        assert!(parse_file("tests/step3/invalid.json").is_err());
    }

    #[test]
    fn test_step_4_valid() {
        assert!(parse_file("tests/step4/valid.json").is_ok());
    }

    #[test]
    fn test_step_4_valid_2() {
        assert!(parse_file("tests/step4/valid2.json").is_ok());
    }

    #[test]
    fn test_step_4_invalid() {
        assert!(parse_file("tests/step4/invalid.json").is_err());
    }
}