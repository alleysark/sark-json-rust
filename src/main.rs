use std::collections::HashMap;

pub mod json_object;

fn main() {
    use json_object::*;

    let mut inner_obj = JSonObject {
        collection: HashMap::new()
    };
    inner_obj.collection.insert(String::from("field_1_1"), JSonValue::AsInt(10));
    inner_obj.collection.insert(String::from("field_1_2"), JSonValue::AsFloat(3.14));

    let mut doc = JSonObject {
        collection: HashMap::new()
    };
    doc.collection.insert(String::from("field_0"), JSonValue::AsInt(10));
    doc.collection.insert(String::from("inner_obj"), JSonValue::AsObject(inner_obj));

    println!("{:#?}", doc);
}