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

}

pub enum JsonValue {
    Object(JsonObject)
}