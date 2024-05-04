use crate::data::*;
use std::io::Error;
use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<Token>) -> Result<JsonData, Error> {
    if tokens.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty Json"));
    }

    let element = parse_element(tokens)?;
    
    if tokens.is_empty() {
        Ok(JsonData {element})
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json"))
    }
}

fn parse_element(tokens: &mut VecDeque<Token>) -> Result<JsonElement, Error> {
    if let Some(value) = parse_value(tokens)? {
        Ok(JsonElement {value})
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json Element"))
    }
}

fn parse_value(tokens: &mut VecDeque<Token>) -> Result<Option<JsonValue>, Error> {
    if Token::True == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
        Ok(Some(JsonValue::True))
    } else if Token::False == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
        Ok(Some(JsonValue::False))
    } else if Token::Null == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
        Ok(Some(JsonValue::Null))
    } else if let Some(object) = parse_object(tokens)? {
        Ok(Some(JsonValue::Object(object)))
    } else if let Some(array) = parse_array(tokens)? {
        Ok(Some(JsonValue::Array(array)))
    } else if let Some(string) = parse_string(tokens)? {
        Ok(Some(JsonValue::String(string)))
    } else if let Some(number) = parse_number(tokens)? {
        Ok(Some(JsonValue::Number(number)))
    } else {
        Ok(None)
    }
}

fn parse_object(tokens: &mut VecDeque<Token>) -> Result<Option<JsonObject>, Error> {
    let mut result = JsonObject {members: vec![]};
    
    if Token::OpenParen == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    if let Some(members) = parse_members(tokens)? {
        result.members = members;
    }

    if Token::CloseParen == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Object"));
    }

    Ok(Some(result))
}

fn parse_array(_tokens: &mut VecDeque<Token>) -> Result<Option<JsonArray>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Array Parsing not implemented"))
}

fn parse_string(_tokens: &mut VecDeque<Token>) -> Result<Option<JsonString>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "String Parsing not implemented"))
}

fn parse_number(_tokens: &mut VecDeque<Token>) -> Result<Option<JsonNumber>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Number Parsing not implemented"))
}

fn parse_members(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonMember>>, Error> {
    if let Ok(Some(member)) = parse_member(tokens) {
        let mut members = vec![member];

        while Token::Comma == *tokens.front().unwrap_or(&Token::Invalid) {
            tokens.pop_front();
            if let Some(member) = parse_member(tokens)? {
                members.push(member);
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Failed to parse members"));
            }
        }
        
        return Ok(Some(members));
    }
    
    Ok(None)
}

#[allow(dead_code)]
fn parse_member(tokens: &mut VecDeque<Token>) -> Result<Option<JsonMember>, Error> {
    let string;

    if let Ok(Some(parsed_string)) = parse_string(tokens) {
        string = parsed_string;
    } else {
        return Ok(None);
    }

    if Token::Colon == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No colon in member"));
    }

    let element = parse_element(tokens)?;
    return Ok(Some(JsonMember {string, element}));
}