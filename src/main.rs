use std::collections::HashMap;

#[derive(Debug)]
struct JSonObject {
    collection: HashMap<String, JSonValue>,
}

#[derive(Debug)]
enum JSonValue {
    as_str(String),
    as_int(i32),
    as_float(f64),
    as_obj(JSonObject),
    as_arr(Vec<JSonValue>),
    as_bool(bool),
    as_null,
}

fn main() {
    let mut inner_obj = JSonObject {
        collection: HashMap::new()
    };
    inner_obj.collection.insert(String::from("field_1_1"), JSonValue::as_int(10));
    inner_obj.collection.insert(String::from("field_1_2"), JSonValue::as_float(3.14));

    let mut doc = JSonObject {
        collection: HashMap::new()
    };
    doc.collection.insert(String::from("field_0"), JSonValue::as_int(10));
    doc.collection.insert(String::from("inner_obj"), JSonValue::as_obj(inner_obj));

    println!("{:#?}", doc);
}