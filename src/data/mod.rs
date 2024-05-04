#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    String(String),
    True,
    False,
    Null,
    Invalid,
}

pub struct JsonData {
    pub element: JsonElement,
}

pub struct JsonElement {
    pub value: JsonValue,
}

pub struct JsonObject {
    pub pairs: Vec<(JsonString, JsonValue)>,
}

pub struct JsonArray {
    pub values: Vec<JsonValue>,
}

pub struct JsonString {

}

pub struct JsonNumber {

}

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