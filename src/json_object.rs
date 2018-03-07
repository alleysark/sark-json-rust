use std::collections::HashMap;

#[derive(Debug)]
pub struct JSonObject {
    pub collection: HashMap<String, JSonValue>,
}

#[derive(Debug)]
pub enum JSonValue {
    as_str(String),
    as_int(i32),
    as_float(f64),
    as_obj(JSonObject),
    as_arr(Vec<JSonValue>),
    as_bool(bool),
    as_null,
}