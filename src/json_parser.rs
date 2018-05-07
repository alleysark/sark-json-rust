use std::io;
use std::io::Read;
use std::fs::File;
use std::str::Chars;
use std::iter::Peekable;
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

// ----- parse number -----
enum JsonParseNumberStage {
    Sign,
    Integer,
    Dot,
    Real,
    ExpNotation,
}

fn parse_number(pk_ch: &mut Peekable<&mut Chars>) -> Result<JsonValue, &'static str> {
    let mut stage: JsonParseNumberStage = JsonParseNumberStage::Sign;

    let mut sign: f64 = 1.0;
    let mut int_part: f64 = 0.0;
    let mut is_float: bool = true;
    let mut real_part: f64 = 0.0;
    let mut exp_part: f64 = 1.0;

    loop {
        match stage {
            JsonParseNumberStage::Sign => {
                if let Some(x) = parse_number_sign(pk_ch) {
                    sign = x;
                    pk_ch.next();
                }
                stage = JsonParseNumberStage::Integer;
            },
            JsonParseNumberStage::Integer =>  {
                // For the strict json syntax, it does not allow the leading zeros,
                // But I am allowing it!
                if let Some(x) = parse_number_digits(pk_ch) {
                    int_part = x;
                    stage = JsonParseNumberStage::Dot;
                }
                else {
                    return Err("parse-number: Failed to parse integer part");
                }
            },
            JsonParseNumberStage::Dot => {
                if pk_ch.peek() == Some(&'.') {
                    // Real part after dot
                    stage = JsonParseNumberStage::Real;
                    pk_ch.next();
                }
                else {
                    // '.' could be nonexistent, then move to exponential notation part without next of iterator
                    is_float = false;
                    stage = JsonParseNumberStage::ExpNotation;
                }
            },
            JsonParseNumberStage::Real => {
                if let Some((x, sz)) = parse_number_digits_with_leading_zeros(pk_ch) {
                    real_part = x * 0.1_f64.powi(sz as i32);
                }
                else {
                    // there is no meaningful real part.
                    // consume following zeros like 1.000
                    while pk_ch.peek().map_or(false, |c| c == &'0') {
                        pk_ch.next();
                    }
                }

                stage = JsonParseNumberStage::ExpNotation;
            },
            JsonParseNumberStage::ExpNotation => {
                if pk_ch.peek() == Some(&'e') || pk_ch.peek() == Some(&'E') {
                    pk_ch.next();

                    // parse the sign of exp notation
                    let mut exp_sign: f64 = 1.0;
                    if let Some(x) = parse_number_sign(pk_ch) {
                        exp_sign = x;
                        pk_ch.next();
                    }
                    
                    let mut exp_amount = 0.0;
                    if let Some(x) = parse_number_digits(pk_ch) {
                        exp_amount = x;
                    }
                    else {
                        return Err("parse-number: error on E notation!");
                    }

                    exp_part = 10.0_f64.powi((exp_sign * exp_amount) as i32);
                }
                break;
            }
        }
    }
    
    if is_float {
        Ok(JsonValue::AsFloat(sign * (int_part + real_part) * exp_part))
    }
    else {
        Ok(JsonValue::AsInt((sign * int_part * exp_part) as i32))
    }
}

fn parse_number_sign(pk_ch: &mut Peekable<&mut Chars>) -> Option<f64> {
    if pk_ch.peek() == Some(&'-') {
        Some(-1.0)
    }
    else if pk_ch.peek() == Some(&'+') {
        Some(1.0)
    }
    else {
        None
    }
}

fn parse_number_digits(pk_ch: &mut Peekable<&mut Chars>) -> Option<f64> {
    parse_number_digits_with_leading_zeros(pk_ch).map_or(None, |(v, _)| Some(v))
}

fn parse_number_digits_with_leading_zeros(pk_ch: &mut Peekable<&mut Chars>) -> Option<(f64, usize)> {
    let mut digits: f64 = 0.0;
    let mut sz: usize = 0;

    while pk_ch.peek().map_or(false, |c| c.is_digit(10)) {
        let d = pk_ch.next().unwrap();
        digits = digits*10.0 + (d.to_digit(10).unwrap() as f64);
        sz += 1;
    }

    if sz != 0 {
        Some((digits, sz))
    }
    else {
        None
    }
}