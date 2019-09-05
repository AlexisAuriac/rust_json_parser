use ll_lexer::lexer;
use ll_lexer::rule::{Rule, RuleTable};

#[derive(Debug, PartialEq, Clone)]
pub enum LexSym {
    TsLBracket,
    TsRBracket,
    TsLCurlyBraces,
    TsRCurlyBraces,
    TsColon,
    TsComma,
    TsQuote,
    TsNbr(i32),
    TsString(String),
    TsBool(bool),
    TsEos,
    TsInvalid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GramSym {
    TsLBracket,
    TsRBracket,
    TsLCurlyBraces,
    TsRCurlyBraces,
    TsColon,
    TsComma,
    TsQuote,
    TsNbr,
    TsString,
    TsBool,
    TsEos,
    TsInvalid,
    NtsValue,
    NtsArrayFirstVal,
    NtsArrayNextVal,
    NtsProperty,
}

pub fn get_rt() -> RuleTable<GramSym> {
    return RuleTable::new(
        vec![(GramSym::NtsValue, false)],
        GramSym::TsEos,
        vec![
            Rule::new(GramSym::NtsValue, GramSym::TsNbr, vec![]),
            Rule::new(GramSym::NtsValue, GramSym::TsString, vec![]),
            Rule::new(GramSym::NtsValue, GramSym::TsBool, vec![]),
            Rule::new(
                GramSym::NtsValue,
                GramSym::TsRBracket,
                vec![(GramSym::NtsValue, true)],
            ),
            Rule::new(
                GramSym::NtsValue,
                GramSym::TsRCurlyBraces,
                vec![
                    (GramSym::NtsProperty, true),
                    (GramSym::TsRCurlyBraces, false),
                ],
            ),
            Rule::new(
                GramSym::NtsProperty,
                GramSym::TsString,
                vec![(GramSym::TsColon, false), (GramSym::NtsValue, false)],
            ),
        ],
    );
}

fn get_nb_spaces(s: &str) -> usize {
    let mut nb_spaces = 0;

    for c in s.chars() {
        if !c.is_whitespace() {
            break;
        }

        nb_spaces += 1;
    }

    return nb_spaces;
}

fn get_string(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    let mut size = 0;

    for c in s.chars() {
        if c == '"' {
            break;
        }

        size += 1;
    }

    return Ok((
        LexSym::TsString(s[..size].to_string()),
        GramSym::TsString,
        size + 2,
    ));
}

fn get_symbol_nbr(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    let mut neg = false;
    let mut nb: i32 = 0;
    let mut size = 0;

    if s.chars().next().unwrap() == '-' {
        neg = true;
    }

    for c in s.chars().skip(if neg { 1 } else { 0 }) {
        if !c.is_digit(10) {
            break;
        }

        if let (nb2, false) = nb.overflowing_mul(10) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        if let (nb2, false) = nb.overflowing_add(c.to_digit(10).unwrap() as i32) {
            nb = nb2;
        } else {
            return Err("Error: Too large number".to_string());
        }

        size += 1;
    }

    if neg {
        nb *= -1;
    }

    return Ok((LexSym::TsNbr(nb), GramSym::TsNbr, size));
}

fn get_bool(s: &str) -> Option<(LexSym, GramSym, usize)> {
    return if s.len() >= 4 && &s[..4] == "true" {
        Some((LexSym::TsBool(true), GramSym::TsBool, 4))
    } else if s.len() >= 5 && &s[..5] == "false" {
        Some((LexSym::TsBool(false), GramSym::TsBool, 5))
    } else {
        None
    };
}

fn get_sym(s: &str) -> Result<(LexSym, GramSym, usize), String> {
    if s.len() == 0 {
        return Ok((LexSym::TsEos, GramSym::TsEos, 0));
    }

    let nb_spaces = get_nb_spaces(s);
    let mut it = s.chars();

    if nb_spaces != 0 {
        it.nth(nb_spaces - 1);
    }

    if let Some((lex_sym, gram_sym, size)) = get_bool(&s[nb_spaces..]) {
        return Ok((lex_sym, gram_sym, size + nb_spaces));
    }

    let (lex_sym, gram_sym, size) = match it.next().unwrap() {
        '[' => (LexSym::TsLBracket, GramSym::TsLBracket, 1),
        ']' => (LexSym::TsRBracket, GramSym::TsRBracket, 1),
        '{' => (LexSym::TsLCurlyBraces, GramSym::TsLCurlyBraces, 1),
        '}' => (LexSym::TsRCurlyBraces, GramSym::TsRCurlyBraces, 1),
        ':' => (LexSym::TsColon, GramSym::TsColon, 1),
        ',' => (LexSym::TsComma, GramSym::TsComma, 1),
        '"' => get_string(&s[nb_spaces + 1..])?,
        '0'...'9' | '-' => get_symbol_nbr(&s[nb_spaces..])?,
        _ => (LexSym::TsInvalid, GramSym::TsInvalid, 1),
    };

    return Ok((lex_sym, gram_sym, size + nb_spaces));
}

fn get_arg() -> Result<String, String> {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return Err(String::from("Give an expression as argument"));
    }

    return Ok(args.remove(1));
}

fn main() -> Result<(), String> {
    let lexed = lexer(get_arg()?, get_rt(), &get_sym)?;

    println!("{:?}", lexed);
    Ok(())
}
