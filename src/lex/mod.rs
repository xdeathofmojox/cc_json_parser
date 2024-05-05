use crate::data::Token;
use std::io::Error;
use std::collections::VecDeque;

pub fn lex(string: &mut &str) -> Result<VecDeque<Token>, Error> {
    let mut result: VecDeque<Token> = VecDeque::new();

    while !string.is_empty() {
        if let Some(token) = lex_open_paren(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_paren(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_open_bracket(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_bracket(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_comma(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_colon(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_string(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_true(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_false(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_null(string)? {
            result.push_back(token);
        } else if let Some(_token) = lex_whitespace(string)? {
            // Do nothing about whitespace for now
            // result.push_back(token);
        } else if let Some(token) = lex_number(string)? {
            result.push_back(token);
        } else {
            return Err(Error::new(std::io::ErrorKind::Other, "Invalid Character"));
        }
    }

    Ok(result)
}

fn lex_open_paren(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('{') {
        *string = &string[1..];
        return Ok(Some(Token::OpenParen));
    }
    Ok(None)
}

fn lex_close_paren(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('}') {
        *string = &string[1..];
        return Ok(Some(Token::CloseParen));
    }
    Ok(None)
}

fn lex_open_bracket(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('[') {
        *string = &string[1..];
        return Ok(Some(Token::OpenBracket));
    }
    Ok(None)
}

fn lex_close_bracket(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(']') {
        *string = &string[1..];
        return Ok(Some(Token::CloseBracket));
    }
    Ok(None)
}

fn lex_comma(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(',') {
        *string = &string[1..];
        return Ok(Some(Token::Comma));
    }
    Ok(None)
}

fn lex_colon(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(':') {
        *string = &string[1..];
        return Ok(Some(Token::Colon));
    }
    Ok(None)
}

fn lex_string(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('"') {
        *string = &string[1..];
        let mut new_string = String::new();
        while !string.starts_with('"') {
            // Handle Escape
            if string.starts_with('\\') {
                *string = &string[1..];
                new_string.push_str(lex_escape(string)?.as_str());
            } else {
                let new_char = string.chars().next().unwrap();
                match new_char {
                    '\u{0020}' ..= '\u{10FFFF}' => {
                        new_string.push(new_char);
                        *string = &string[1..];
                    }
                    _ => {
                        return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Unicode Character in String"))
                    }
                }
            }
        }
        *string = &string[1..];
        return Ok(Some(Token::String(new_string)));
    }
    Ok(None)
}

fn lex_escape(string: &mut &str) -> Result<String, Error> {
    if string.starts_with('"') {
        *string = &string[1..];
        Ok(String::from("\\\""))
    } else if string.starts_with('\\') {
        *string = &string[1..];
        Ok(String::from("\\\\"))
    } else if string.starts_with('/') {
        *string = &string[1..];
        Ok(String::from("\\/"))
    } else if string.starts_with('b') {
        *string = &string[1..];
        Ok(String::from("\\b"))
    } else if string.starts_with('f') {
        *string = &string[1..];
        Ok(String::from("\\f"))
    } else if string.starts_with('n') {
        *string = &string[1..];
        Ok(String::from("\\n"))
    } else if string.starts_with('r') {
        *string = &string[1..];
        Ok(String::from("\\r"))
    } else if string.starts_with('t') {
        *string = &string[1..];
        Ok(String::from("\\t"))
    } else if string.starts_with('u') {
        *string = &string[1..];
        Ok(lex_escape_hex(string)?)
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Escape Character"))
    }
}

fn lex_escape_hex(string: &mut &str) -> Result<String, Error> {
    if string.len() < 4 {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Not enough hex characters in escape"))
    } else {
        let mut new_string = String::from("\\u");
        let mut chars = string.chars();
        for _ in 0..4 {
            let new_char = chars.next().unwrap();
            match new_char {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    new_string.push(new_char);
                }
                _ => {
                    return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Escape Hex Character"))
                }
            }
        }
        *string = &string[4..];
        Ok(new_string)
    }
}

fn lex_true(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("true") {
        *string = &string[4..];
        return Ok(Some(Token::True));
    }
    Ok(None)
}

fn lex_false(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("false") {
        *string = &string[5..];
        return Ok(Some(Token::False));
    }
    Ok(None)
}

fn lex_null(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("null") {
        *string = &string[4..];
        return Ok(Some(Token::Null));
    }
    Ok(None)
}

fn lex_whitespace(string: &mut &str) -> Result<Option<Token>, Error> {
    let mut found_whitespace = false;
    while string.starts_with(' ') || string.starts_with('\n') || string.starts_with('\t') || string.starts_with('\r') {
        *string = &string[1..];
        found_whitespace = true;
    }

    if found_whitespace {
        return Ok(Some(Token::Whitespace));
    }
    Ok(None)
}

fn lex_number(string: &mut &str) -> Result<Option<Token>, Error> {
    let char = string.chars().next();
    match char {
        Some('-') => {
            *string = &string[1..];
            Ok(Some(Token::SignNeg))
        },
        Some('+') => {
            *string = &string[1..];
            Ok(Some(Token::SignPos))
        },
        Some('0'..='9') => {
            *string = &string[1..];
            Ok(Some(Token::Digit(char.unwrap() as u8)))
        },
        Some('.') => {
            *string = &string[1..];
            Ok(Some(Token::FractionMarker))
        },
        Some('e') | Some('E') => {
            *string = &string[1..];
            Ok(Some(Token::ExponentMarker))
        }
        _ => {
            Ok(None)
        }
    }
}