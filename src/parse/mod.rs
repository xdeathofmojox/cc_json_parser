use crate::data::*;
use std::io::Error;
use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<Token>) -> Result<JsonData, Error> {
    if tokens.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty Json"));
    }

    if let Some(element) = parse_element(tokens)? {
        Ok(JsonData {element})
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json"))
    }
}

fn parse_element(tokens: &mut VecDeque<Token>) -> Result<Option<JsonElement>, Error> {
    if let Some(value) = parse_value(tokens)? {
        Ok(Some(JsonElement {value}))
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
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json Element"))
    }
}

fn parse_object(tokens: &mut VecDeque<Token>) -> Result<Option<JsonObject>, Error> {
    let result = JsonObject {members: vec![]};
    
    if Token::OpenParen == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    // parse_members(tokens)?;

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

#[allow(dead_code)]
fn parse_members(_tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonMember>>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Members Parsing not implemented"))
}

#[allow(dead_code)]
fn parse_member(_tokens: &mut VecDeque<Token>) -> Result<Option<JsonMember>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Member Parsing not implemented"))
}