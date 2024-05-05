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
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json: Additional Data Left Over"))
    }
}

fn parse_element(tokens: &mut VecDeque<Token>) -> Result<JsonElement, Error> {
    if let Some(value) = parse_value(tokens)? {
        Ok(JsonElement {value})
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json Element"))
    }
}

fn parse_elements(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonElement>>, Error> {
    if let Ok(element) = parse_element(tokens) {
        let mut elements = vec![element];

        while let Some(&Token::Comma) = tokens.front() {
            tokens.pop_front();
            let element = parse_element(tokens)?;
            elements.push(element);
        }
        
        return Ok(Some(elements));
    }
    
    Ok(None)
}

fn parse_value(tokens: &mut VecDeque<Token>) -> Result<Option<JsonValue>, Error> {
    if let Some(&Token::True) = tokens.front() {
        tokens.pop_front();
        Ok(Some(JsonValue::True))
    } else if let Some(&Token::False) = tokens.front() {
        tokens.pop_front();
        Ok(Some(JsonValue::False))
    } else if let Some(&Token::Null) = tokens.front() {
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
    
    if let Some(&Token::OpenParen) = tokens.front() {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    if let Some(members) = parse_members(tokens)? {
        result.members = members;
    }

    println!("{:?}", *tokens);
    if let Some(&Token::CloseParen) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No Closing Paren on Object"));
    }

    Ok(Some(result))
}

fn parse_array(tokens: &mut VecDeque<Token>) -> Result<Option<JsonArray>, Error> {
    let mut result = JsonArray {elements: vec![]};
    
    if let Some(&Token::OpenBracket) = tokens.front() {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    if let Some(elements) = parse_elements(tokens)? {
        result.elements = elements;
    }

    if let Some(&Token::CloseBracket) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No Closing Bracket on  Array"));
    }

    Ok(Some(result))
}

fn parse_string(tokens: &mut VecDeque<Token>) -> Result<Option<JsonString>, Error> {
    if let Some(&Token::String(_)) = tokens.front() {
        if let Token::String(string) = tokens.pop_front().unwrap() {
            return Ok(Some(JsonString { string }));
        } else {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "String not parsed correctly"));
        }
    }

    Ok(None)
}

fn parse_number(_tokens: &VecDeque<Token>) -> Result<Option<JsonNumber>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Number Parsing not implemented"))
}

fn parse_members(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonMember>>, Error> {
    if let Ok(Some(member)) = parse_member(tokens) {
        let mut members = vec![member];

        while let Some(&Token::Comma) = tokens.front() {
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

fn parse_member(tokens: &mut VecDeque<Token>) -> Result<Option<JsonMember>, Error> {
    let string;

    if let Ok(Some(parsed_string)) = parse_string(tokens) {
        string = parsed_string;
    } else {
        return Ok(None);
    }

    if let Some(&Token::Colon) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No colon in member"));
    }

    let element = parse_element(tokens)?;
    Ok(Some(JsonMember {string, element}))
}