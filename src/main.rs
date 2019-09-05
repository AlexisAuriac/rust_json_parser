use std::collections::HashMap;

use ll_lexer::lexer;

mod rule_table;
mod symbol;

use rule_table::get_rt;
use symbol::{get_sym, LexSym};

fn get_arg() -> Result<String, String> {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return Err(String::from("Give an expression as argument"));
    }

    return Ok(args.remove(1));
}

enum JsonValue {
    Nbr(i32),
    Str(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn print_node_prof(prof: u32) {
    for _ in 0..prof {
        print!("\t");
    }
}

fn print_json_array(arr: &Vec<JsonValue>, prof: u32) {
    if arr.len() == 0 {
        print_node_prof(prof);
        print!("[]");
        return;
    }

    print_node_prof(prof);
    println!("[");

    for val in arr {
        print_json_value(val, prof + 1);
        println!(",");
    }

    print_node_prof(prof);
    print!("]");
}

fn print_json_value(val: &JsonValue, prof: u32) {
    match val {
        JsonValue::Nbr(n) => {
            print_node_prof(prof);
            print!("{}", n);
        }
        JsonValue::Str(s) => {
            print_node_prof(prof);
            print!("{}", s);
        }
        JsonValue::Bool(b) => {
            print_node_prof(prof);
            print!("{}", b);
        }
        JsonValue::Array(content) => print_json_array(content, prof),
        _ => print!("nope"),
    }
}

impl std::fmt::Debug for JsonValue {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        print_json_value(self, 0);
        Ok(())
    }
}

fn parse_array(lexed: &mut std::slice::Iter<LexSym>) -> JsonValue {
    let mut opened_bracket = 1;
    let mut content: Vec<JsonValue> = vec![];

    loop {
        match lexed.next().unwrap() {
            LexSym::TsRBracket => {
                if opened_bracket == 1 {
                    break;
                } else {
                    opened_bracket -= 1;
                }
            }
            LexSym::TsLBracket => opened_bracket += 1,
            LexSym::TsComma => (),
            _ => content.push(parse_value(lexed)),
        }
    }

    return JsonValue::Array(content);
}

fn parse_value(lexed: &mut std::slice::Iter<LexSym>) -> JsonValue {
    return match lexed.next().unwrap() {
        LexSym::TsNbr(n) => JsonValue::Nbr(*n),
        LexSym::TsString(s) => JsonValue::Str(s.to_string()),
        LexSym::TsBool(b) => JsonValue::Bool(*b),
        LexSym::TsLBracket => parse_array(lexed),
        _ => unimplemented!(),
    };
}

fn parse(lexed: &Vec<LexSym>) -> JsonValue {
    return parse_value(&mut lexed.iter());
}

fn main() -> Result<(), String> {
    let lexed = lexer(get_arg()?, get_rt(), &get_sym)?;
    let parsed = parse(&lexed);

    println!("{:?}", &parsed);
    Ok(())
}
