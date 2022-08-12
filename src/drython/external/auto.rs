use crate::drython::types::Token;
use super::{RegisteredFunction, RegisteredVariable, register_custom_function};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    register_custom_function!(functions, "print", print);    
}

pub fn register_vars(variables: &mut Vec<RegisteredVariable>)
{

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
