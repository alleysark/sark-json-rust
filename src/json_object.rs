use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonObject {
    pub collection: HashMap<String, JsonValue>,
}

#[derive(Debug)]
pub enum JsonValue {
    AsStr(String),
    AsInt(i32),
    AsFloat(f64),
    AsObject(JsonObject),
    AsArray(Vec<JsonValue>),
    AsBool(bool),
    AsNull,
}