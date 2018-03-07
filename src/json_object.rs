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

impl JsonObject {
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.collection.get(key)
    }

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    // if name exists, the value is updated, and the old value is returned (like HashMap of std::collections)
    pub fn insert(&mut self, name: &str, val: JsonValue) -> Option<JsonValue> {
        self.collection.insert(String::from(name), val)
    }
}