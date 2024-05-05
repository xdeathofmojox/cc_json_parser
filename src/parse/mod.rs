use crate::data::*;
use std::io::Error;
use std::collections::VecDeque;

pub fn parse(tokens: &mut VecDeque<Token>) -> Result<JsonData, Error> {
    if tokens.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty Json"));
    }

    if let Some(element) = parse_element(tokens)? {
        if tokens.is_empty() {
            Ok(JsonData {element})
        } else {
            Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json: Additional Data Left Over"))
        }
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid Json: No Element"))
    }
}

fn parse_element(tokens: &mut VecDeque<Token>) -> Result<Option<JsonElement>, Error> {
    if let Some(value) = parse_value(tokens)? {
        Ok(Some(JsonElement {value}))
    } else {
        Ok(None)
    }
}

fn parse_elements(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonElement>>, Error> {
    if let Some(element) = parse_element(tokens)? {
        let mut elements = vec![element];

        while let Some(&Token::Comma) = tokens.front() {
            tokens.pop_front();
            if let Some(element) = parse_element(tokens)? {
                elements.push(element);
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Failed to parse element"));
            }
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
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No Closing Bracket on Array"));
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

fn parse_number(tokens: &mut VecDeque<Token>) -> Result<Option<JsonNumber>, Error> {
    if let Some(integer) = parse_integer(tokens)? {

        Ok(Some(JsonNumber {
            integer,
            fraction: None,
            exponent: None,
        }))
    } else {
        Ok(None)
    }
}

fn parse_integer(tokens: &mut VecDeque<Token>) -> Result<Option<i64>, Error> {
    let mut counter = 0;
    let mut neg_sign = false;
    let mut found_nums = false;
    let mut int_value: i64 = 0;
    if let Some(&Token::SignNeg) = tokens.front() {
        tokens.pop_front();
        neg_sign = true;
    }

    while let Some(&Token::Digit(val)) = tokens.front() {
        tokens.pop_front();
        found_nums = true;
        let new_num = int_value.checked_add((val as i64) * 10_i64.pow(counter));
        if new_num.is_some() {
            int_value = new_num.unwrap();
        } else {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Integer too large"));
        }
        counter += 1;
    }

    if neg_sign && !found_nums {
        Err(Error::new(std::io::ErrorKind::InvalidData, "No digits following sign"))
    } else if found_nums {
        if neg_sign {
            int_value = -int_value;
        }
        Ok(Some(int_value))
    } else {
        Ok(None)
    }
}

#[allow(dead_code)]
fn parse_fraction(_tokens: &VecDeque<Token>) -> Result<Option<u64>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Number Parsing not implemented"))
}

#[allow(dead_code)]
fn parse_exponent(_tokens: &VecDeque<Token>) -> Result<Option<i64>, Error> {
    Err(Error::new(std::io::ErrorKind::InvalidData, "Number Parsing not implemented"))
}

fn parse_members(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonMember>>, Error> {
    if let Some(member) = parse_member(tokens)? {
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

    if let Some(parsed_string) = parse_string(tokens)? {
        string = parsed_string;
    } else {
        return Ok(None);
    }

    if let Some(&Token::Colon) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "No colon in member"));
    }

    if let Some(element) = parse_element(tokens)? {
        Ok(Some(JsonMember {string, element}))
    } else {
        Err(Error::new(std::io::ErrorKind::InvalidData, "No element for string"))
    }
}