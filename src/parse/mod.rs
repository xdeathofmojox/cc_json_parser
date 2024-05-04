use crate::data::{Token, JsonObject, JsonValue, JsonElement, JsonData};
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
    if let Some(object) = parse_object(tokens)? {
        Ok(Some(JsonValue::Object(object)))
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

    // parse_members

    if Token::CloseParen == *tokens.front().unwrap_or(&Token::Invalid) {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Object"));
    }

    Ok(Some(result))
}