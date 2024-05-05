#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    Number {
        integer: i64,
        fraction: Option<u64>,
        exponent: Option<i64>
    },
    String(String),
    True,
    False,
    Null,
    Whitespace,
}

#[derive(Debug)]
pub struct JsonData {
    pub element: JsonElement,
}

#[derive(Debug)]
pub struct JsonElement {
    pub value: JsonValue,
}

#[derive(Debug)]
pub struct JsonObject {
    pub members: Vec<JsonMember>,
}

#[derive(Debug)]
pub struct JsonMember {
    pub string: JsonString,
    pub element: JsonElement,
}

#[derive(Debug)]
pub struct JsonArray {
    pub elements: Vec<JsonElement>,
}

#[derive(Debug)]
pub struct JsonNumber {
    pub integer: i64,
    pub fraction: Option<u64>,
    pub exponent: Option<i64>,
}

#[derive(Debug)]
pub struct JsonString {
    pub string: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    True,
    False,
    Null
}