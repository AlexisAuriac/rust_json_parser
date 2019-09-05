use ll_lexer::lexer;

mod json_value;
mod parse;
mod rule_table;
mod symbol;

use parse::parse;
use rule_table::get_rt;
use symbol::get_sym;

fn get_arg() -> Result<String, String> {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        return Err(String::from("Give an expression as argument"));
    }

    return Ok(args.remove(1));
}

fn main() -> Result<(), String> {
    let lexed = lexer(get_arg()?, get_rt(), &get_sym)?;
    let parsed = parse(&lexed);

    println!("{:?}", &parsed);
    Ok(())
}
