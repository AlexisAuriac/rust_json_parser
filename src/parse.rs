use std::collections::HashMap;

use crate::json_value::JsonValue;
use crate::symbol::LexSym;

fn parse_array(lexed: &mut std::slice::Iter<LexSym>) -> JsonValue {
    let mut content: Vec<JsonValue> = vec![];

    let mut it2 = lexed.clone();

    if let Some(LexSym::TsRBracket) = it2.next() {
        return JsonValue::Array(content);
    }

    loop {
        content.push(parse_value(lexed));
        if let Some(LexSym::TsComma) = lexed.next() {
        } else {
            break;
        }
    }

    return JsonValue::Array(content);
}

fn parse_object(lexed: &mut std::slice::Iter<LexSym>) -> JsonValue {
    let mut props: HashMap<String, JsonValue> = HashMap::new();

    let mut it2 = lexed.clone();

    if let Some(LexSym::TsRCurlyBraces) = it2.next() {
        return JsonValue::Object(props);
    }

    loop {
        let key = if let Some(LexSym::TsString(s)) = lexed.next() {
            s
        } else {
            unimplemented!();
        };

        lexed.next();

        let val = parse_value(lexed);

        props.insert(key.to_string(), val);

        if let Some(LexSym::TsComma) = lexed.next() {
        } else {
            break;
        }
    }

    return JsonValue::Object(props);
}

fn parse_value(lexed: &mut std::slice::Iter<LexSym>) -> JsonValue {
    return match lexed.next().unwrap() {
        LexSym::TsNbr(n) => JsonValue::Nbr(*n),
        LexSym::TsString(s) => JsonValue::Str(s.to_string()),
        LexSym::TsBool(b) => JsonValue::Bool(*b),
        LexSym::TsLBracket => parse_array(lexed),
        LexSym::TsLCurlyBraces => parse_object(lexed),
        _ => unimplemented!(),
    };
}

pub fn parse(lexed: &Vec<LexSym>) -> JsonValue {
    return parse_value(&mut lexed.iter());
}
