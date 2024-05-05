use std::fs::File;
use std::io::{Read, Error};

mod lex;
mod data;
mod parse;

use data::JsonData;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn parse_file(filename: &str) -> Result<JsonData, Error> {
    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut tokens = lex::lex(&mut s.as_str())?;
    let json_data = parse::parse(&mut tokens)?;
    Ok(json_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::data::*;

    #[test]
    fn test_step_1_valid() {
        let expected = JsonData {
            element: JsonElement {
                value: JsonValue::Object(
                    JsonObject {
                        members: vec![]
                    }
                )
            }
        };

        let json_result = parse_file("tests/step1/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_1_invalid() {
        let json_result = parse_file("tests/step1/invalid.json");
        assert!(json_result.is_err());
        assert!(json_result.err().unwrap().to_string() == "Empty Json");
    }

    #[test]
    fn test_step_2_valid() {
        let expected = JsonData {
            element: JsonElement {
                value: JsonValue::Object(
                    JsonObject {
                        members: vec![
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key")
                                },
                                element: JsonElement {
                                    value: JsonValue::String(JsonString {
                                        string: String::from("value")
                                    })
                                }
                            }
                        ]
                    }
                )
            }
        };

        let json_result = parse_file("tests/step2/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_2_invalid() {
        let json_result = parse_file("tests/step2/invalid.json");
        assert!(json_result.is_err());
        assert!(json_result.err().unwrap().to_string() == "Failed to parse members");
    }

    #[test]
    fn test_step_2_valid_2() {
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