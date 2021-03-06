use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct JsonObject {
    collection: HashMap<String, JsonValue>,
}

#[derive(Debug)]
pub enum JsonValue {
    AsString(String),
    AsInt(i32),
    AsFloat(f64),
    AsObject(JsonObject),
    AsArray(Vec<JsonValue>),
    AsBool(bool),
    AsNull,
}

// ----- impl JsonObject methods -----
impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject {
            collection: HashMap::new()
        }
    }

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

// ----- impl JsonObject Clone trait -----
impl Clone for JsonObject {
    fn clone(&self) -> JsonObject {
        JsonObject {
            collection: self.collection.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.collection.clone_from(&source.collection);
    }
}

// ----- define JsonObject construction macro -----
#[macro_export]
macro_rules! json_obj (
    { $($key:expr => $value:expr), * } => {{
        let mut obj = JsonObject::new();
        $(
            obj.insert($key, JsonValue::from($value));
        )*
        obj
    }};

    { $($key:expr => $value:expr,) * } => {
        json_obj!{ $($key => $value), * }
    }
);

#[macro_export]
macro_rules! json_arr (
    ( $elem:expr; $n:expr ) => {
        std::vec::from_elem(JsonValue::from($elem), $n)
    };
    
    ( $($x:expr), * ) => {
        <[_]>::into_vec(std::boxed::Box::new([$(JsonValue::from($x)), *]))
    };
    
    ( $($x:expr,) * ) => {
        json_arr![$($x), *]
    }
);

// ----- impl JsonValue Clone trait -----
impl Clone for JsonValue {
    fn clone(&self) -> JsonValue {
        match *self {
            JsonValue::AsString(ref s) => JsonValue::AsString(s.clone()),
            JsonValue::AsInt(i) => JsonValue::AsInt(i),
            JsonValue::AsFloat(f) => JsonValue::AsFloat(f),
            JsonValue::AsObject(ref obj) => JsonValue::AsObject(obj.clone()),
            JsonValue::AsArray(ref arr) => JsonValue::AsArray(arr.clone()),
            JsonValue::AsBool(b) => JsonValue::AsBool(b),
            JsonValue::AsNull => JsonValue::AsNull,
        }
    }
}

// ----- impl JsonValue From trait -----
impl<'a> From<&'a String> for JsonValue {
    fn from(src: &'a String) -> Self {
        JsonValue::AsString(src.clone())
    }
}
impl From<String> for JsonValue {
    fn from(src: String) -> Self {
        JsonValue::AsString(src)
    }
}
impl<'a> From<&'a str> for JsonValue {
    fn from(s: &'a str) -> Self {
        JsonValue::AsString(String::from(s))
    }
}
impl From<char> for JsonValue {
    fn from(c: char) -> Self {
        JsonValue::AsString(c.to_string())
    }
}
impl From<i32> for JsonValue {
    fn from(i: i32) -> Self {
        JsonValue::AsInt(i)
    }
}
impl From<f64> for JsonValue {
    fn from(f: f64) -> Self {
        JsonValue::AsFloat(f)
    }
}
impl<'a> From<&'a JsonObject> for JsonValue {
    fn from(obj: &'a JsonObject) -> Self {
        JsonValue::AsObject(obj.clone())
    }
}
impl From<JsonObject> for JsonValue {
    fn from(obj: JsonObject) -> Self {
        JsonValue::AsObject(obj)
    }
}
impl<'a> From<&'a Vec<JsonValue>> for JsonValue {
    fn from(arr: &'a Vec<JsonValue>) -> Self {
        JsonValue::AsArray(arr.clone())
    }
}
impl From<Vec<JsonValue>> for JsonValue {
    fn from(arr: Vec<JsonValue>) -> Self {
        JsonValue::AsArray(arr)
    }
}
impl From<bool> for JsonValue {
    fn from(b: bool) -> Self {
        JsonValue::AsBool(b)
    }
}