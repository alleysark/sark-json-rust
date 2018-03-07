use std::collections::HashMap;

pub mod json_object;

fn main() {
    use json_object::*;

    let mut inner_obj = JsonObject {
        collection: HashMap::new()
    };
    inner_obj.collection.insert(String::from("field_1_1"), JsonValue::AsInt(10));
    inner_obj.collection.insert(String::from("field_1_2"), JsonValue::AsFloat(3.14));

    let mut doc = JsonObject {
        collection: HashMap::new()
    };
    doc.collection.insert(String::from("field_0"), JsonValue::AsInt(10));
    doc.collection.insert(String::from("inner_obj"), JsonValue::AsObject(inner_obj));

    println!("{:#?}", doc);
}