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
        } else if let Some(token) = lex_whitespace(string)? {
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
    Ok(None)
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
    while string.starts_with(" ") || string.starts_with("\n") || string.starts_with("\t") || string.starts_with("\r") {
        *string = &string[1..];
    }
    Ok(None)
}

fn lex_number(string: &mut &str) -> Result<Option<Token>, Error> {
    Ok(None)
}