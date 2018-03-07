use std::collections::HashMap;

#[derive(Debug)]
pub struct JSonObject {
    pub collection: HashMap<String, JSonValue>,
}

#[derive(Debug)]
pub enum JSonValue {
    AsStr(String),
    AsInt(i32),
    AsFloat(f64),
    AsObject(JSonObject),
    AsArray(Vec<JSonValue>),
    AsBool(bool),
    AsNull,
}