use crate::types::Token;
use super::{expect, RegisteredFunction, RegisteredVariable, register_custom_function};

pub fn register_functs(functions: &mut Vec<RegisteredFunction>)
{
    register_custom_function!(functions, "print", print);    
}

pub fn register_vars(_variables: &mut Vec<RegisteredVariable>)
{

}

fn print(args: Vec<Token>) -> Result<Option<Token>, String>
{
    match expect(&args, &[None])
    {
        Err(error) => { return Err(error); }
        Ok(_) => ()
    }

    match &args[0]
    {
        Token::Int(i) => println!("{}", i),
        Token::Float(f) => println!("{}", f),
        Token::String(s) => println!("{}", s),
        Token::Char(c) => println!("{}", c),
        Token::Bool(b) => println!("{}", b),
        Token::Collection(c) => println!("{:?}", c),
        _ => { return Err(format!("Cannot print a variable of this type: {:?}", args[0])); }
    }

    Ok(None)
}
