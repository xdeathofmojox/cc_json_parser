#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    SignPos,
    SignNeg,
    Digit(u8),
    FractionMarker,
    ExponentMarker,
    String(String),
    True,
    False,
    Null,
    Whitespace,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonData {
    pub element: JsonElement,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonElement {
    pub value: JsonValue,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonObject {
    pub members: Vec<JsonMember>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonMember {
    pub string: JsonString,
    pub element: JsonElement,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonArray {
    pub elements: Vec<JsonElement>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonNumber {
    pub integer: i64,
    pub fraction: Option<u64>,
    pub exponent: Option<i64>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct JsonString {
    pub string: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    True,
    False,
    Null
}