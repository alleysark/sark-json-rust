use std::io;
use std::io::Read;
use std::fs::File;
use json_object::*;

pub fn parse(file_name: &str) -> Result<JsonObject, io::Error> {
    let mut f = File::open(file_name)?;

    let mut json_str = String::new();
    f.read_to_string(&mut json_str)?;

    println!("json_str: {}", json_str);

    Ok(json_obj!{
        "filename" => file_name,
        "content" => json_str,
    })
}