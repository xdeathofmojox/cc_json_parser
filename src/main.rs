use std::env;
use std::fs::File;
use std::io::{self, Error, BufReader, BufRead};
use std::process::ExitCode;


mod lex;
mod data;
mod parse;

use data::JsonData;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let filenames = parse_args(args);
    let mut status = 0;

    if filenames.is_empty() {
        let stdin = io::stdin();
        let result = json_valid(&mut stdin.lock());
        if result.is_err() {
            println!("Invalid: stdin - {:?}", result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: stdin");
        }
    }

    for filename in filenames {
        let result = handle_file(filename.as_str());
        if result.is_err() {
            println!("Invalid: {:?} - {:?}", filename, result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: {:?}", filename);
        }
    }

    ExitCode::from(status)
}

fn parse_args(args: Vec<String>) -> Vec<String> {
    let mut file_result: Vec<String> = vec![];
    for arg in args {
        file_result.push(arg);
    }

    file_result
}

fn json_valid<R: BufRead>(reader: &mut R) -> Result<JsonData, Error> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let mut tokens = lex::lex(&mut s.as_str())?;
    let json_data = parse::parse(&mut tokens)?;
    Ok(json_data)
}

fn handle_file(filename: &str) -> Result<JsonData, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    json_valid(&mut reader)
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

        let json_result = handle_file("tests/step1/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_1_invalid() {
        let json_result = handle_file("tests/step1/invalid.json");
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

        let json_result = handle_file("tests/step2/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_2_invalid() {
        let json_result = handle_file("tests/step2/invalid.json");
        assert!(json_result.is_err());
        assert!(json_result.err().unwrap().to_string() == "Failed to parse members");
    }

    #[test]
    fn test_step_2_valid_2() {
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
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key2")
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

        let json_result = handle_file("tests/step2/valid2.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_2_invalid_2() {
        let json_result = handle_file("tests/step2/invalid2.json");
        assert!(json_result.is_err());
        assert!(json_result.err().unwrap().to_string() == "Invalid Character");
    }

    #[test]
    fn test_step_3_valid() {
        let expected = JsonData {
            element: JsonElement {
                value: JsonValue::Object(
                    JsonObject {
                        members: vec![
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key1")
                                },
                                element: JsonElement {
                                    value: JsonValue::True
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key2")
                                },
                                element: JsonElement {
                                    value: JsonValue::False
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key3")
                                },
                                element: JsonElement {
                                    value: JsonValue::Null
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key4")
                                },
                                element: JsonElement {
                                    value: JsonValue::String(JsonString {
                                        string: String::from("value"),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key5")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber {
                                        integer: 101,
                                        fraction: None,
                                        exponent: None,
                                    })
                                }
                            },
                        ]
                    }
                )
            }
        };

        let json_result = handle_file("tests/step3/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);    }

    #[test]
    fn test_step_3_invalid() {
        let json_result = handle_file("tests/step3/invalid.json");
        assert!(json_result.is_err());
        assert!(json_result.err().unwrap().to_string() == "Invalid Character");
    }

    #[test]
    fn test_step_4_valid() {
        assert!(handle_file("tests/step4/valid.json").is_ok());
    }

    #[test]
    fn test_step_4_valid_2() {
        assert!(handle_file("tests/step4/valid2.json").is_ok());
    }

    #[test]
    fn test_step_4_invalid() {
        assert!(handle_file("tests/step4/invalid.json").is_err());
    }

    #[test]
    fn test_step_5_valid() {
        let expected = JsonData {
            element: JsonElement {
                value: JsonValue::Object(
                    JsonObject {
                        members: vec![
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key1")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: None, 
                                        exponent: None,
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key2")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: Some(0), 
                                        exponent: None,
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key3")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: Some(0), 
                                        exponent: Some(10),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key4")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: Some(0), 
                                        exponent: Some(-10),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key5")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: Some(0), 
                                        exponent: Some(10),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key6")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 999, 
                                        fraction: Some(9999), 
                                        exponent: Some(999),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key7")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 0, 
                                        fraction: Some(0), 
                                        exponent: Some(0),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key8")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 100, 
                                        fraction: None, 
                                        exponent: Some(8),
                                    })
                                }
                            },
                            JsonMember {
                                string: JsonString {
                                    string: String::from("key9")
                                },
                                element: JsonElement {
                                    value: JsonValue::Number(JsonNumber { 
                                        integer: 9_223_372_036_854_775_807i64, 
                                        fraction: None, 
                                        exponent: None,
                                    })
                                }
                            },
                        ]
                    }
                )
            }
        };

        let json_result = handle_file("tests/step5/valid.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }

    #[test]
    fn test_step_5_valid_2() {
        let expected = JsonData {
            element: JsonElement {
                value: JsonValue::Array(
                    JsonArray {
                        elements: vec![
                            JsonElement {
                                value: JsonValue::True
                            },
                            JsonElement {
                                value: JsonValue::False
                            },
                            JsonElement {
                                value: JsonValue::Null
                            },
                        ]
                    }
                )
            }
        };
        let json_result = handle_file("tests/step5/valid2.json");
        assert!(json_result.is_ok());
        assert_eq!(json_result.unwrap(), expected);
    }
}