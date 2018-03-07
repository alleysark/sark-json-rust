
#[macro_use]
pub mod json_object;

fn main() {
    use json_object::*;

    let mut inner_obj = JsonObject::new();
    inner_obj.insert("field_1_1", JsonValue::AsInt(10));
    inner_obj.insert("field_1_2", JsonValue::AsFloat(3.14));

    let mut doc = JsonObject::new();
    doc.insert("field_0", JsonValue::AsInt(10));
    doc.insert("inner_obj", JsonValue::AsObject(inner_obj));

    println!("{:#?}", doc);
    println!("{:#?}", doc.get("inner_obj"));
    println!("length of doc: {}", doc.len());

    // using macro
    let mut obj = json_obj!{
        "age" => JsonValue::AsInt(28),
        "favorites" => JsonValue::AsArray(vec![
            JsonValue::AsObject(json_obj!{
                "type" => JsonValue::AsStr(String::from("color")),
                "target" => JsonValue::AsStr(String::from("violet"))
            }),
            JsonValue::AsFloat(0.1),
            JsonValue::AsFloat(0.2),
            JsonValue::AsFloat(0.3),
        ])
    };
    println!("{:#?}", obj);
}