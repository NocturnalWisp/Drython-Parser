use crate::drython::types::Token;

pub fn register_functs() -> Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)>
{
    let mut functions: Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)> = Vec::new();

    functions.push(("print".to_string(), print));

    functions
}

pub fn register_vars() -> Vec<(std::string::String, Token)>
{
    let mut vars: Vec<(std::string::String, Token)> = Vec::new();


    vars
}

fn print(args: Vec<Token>) -> Option<Token>
{
    if args.len() != 1 { return None; }
    
    match &args[0]
    {
        Token::Int(i) => println!("{}", i),
        Token::Float(f) => println!("{}", f),
        Token::String(s) => println!("{}", s),
        Token::Collection(c) => println!("{:?}", c),
        _ => ()
    }

    return Some(Token::Error(format!("Cannot print a variable of this type: {:?}", args[0])));
}
