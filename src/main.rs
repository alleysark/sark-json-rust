
#[macro_use]
pub mod json_object;

fn main() {
    use json_object::*;

    let mut inner_obj = JsonObject::new();
    inner_obj.insert("field_1_1", JsonValue::from(10));
    inner_obj.insert("field_1_2", JsonValue::from(3.14));

    let mut obj = JsonObject::new();
    obj.insert("field_0", JsonValue::from(10));
    obj.insert("inner_obj", JsonValue::from(inner_obj));

    println!("obj: {:#?}", obj);
    println!("inner_obj: {:#?}", obj.get("inner_obj"));
    println!("length of obj: {}", obj.len());
    println!("");

    // using macro
    let obj = json_obj!{
        "age" => JsonValue::from(28),
        "favorites" => JsonValue::from(vec![
            JsonValue::from(json_obj!{
                "type" => JsonValue::from("color"),
                "target" => JsonValue::from("violet")
            }),
            JsonValue::from(0.1),
            JsonValue::from(0.2),
            JsonValue::from(0.3),
        ])
    };
    println!("obj: {:#?}", obj);

    let cloned_obj = obj.clone();
    println!("cloned_obj: {:#?}", cloned_obj);
}