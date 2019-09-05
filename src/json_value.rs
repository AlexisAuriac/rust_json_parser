use std::collections::HashMap;

pub enum JsonValue {
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

fn print_json_object(props: &HashMap<String, JsonValue>, prof: u32) {
    if props.len() == 0 {
        print_node_prof(prof);
        print!("{{}}");
        return;
    }

    print_node_prof(prof);
    println!("{{");

    for (key, val) in props {
        print_node_prof(prof + 1);
        print!("\"{}\": ", key);
        print_json_value(val, prof + 1);
        println!(",");
    }

    print_node_prof(prof);
    print!("}}");
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
        JsonValue::Object(props) => print_json_object(props, prof),
    }
}

impl std::fmt::Debug for JsonValue {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        print_json_value(self, 0);
        Ok(())
    }
}
