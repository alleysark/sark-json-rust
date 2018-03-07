
#[derive(Debug)]
struct JSonObject {
    name: String,
    value: Box<JSonValue>,
}

#[derive(Debug)]
enum JSonValue {
    as_str(String),
    as_int(i32),
    as_float(f64),
    as_obj(Box<JSonObject>),
    as_arr(Vec<JSonValue>),
    as_bool(bool),
    as_null,
}

fn main() {
    let obj = JSonObject {
        name: String::from("field_name"),
        value: Box::new(JSonValue::as_obj(Box::new(
            JSonObject {
                name: String::from("inner_field_name"),
                value: Box::new(JSonValue::as_arr(vec![
                    JSonValue::as_int(10),
                    JSonValue::as_float(3.14),
                ])),
            }
        ))),
    };

    println!("{:#?}", obj);
}
