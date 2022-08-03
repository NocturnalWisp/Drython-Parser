use crate::drython::types::Token::{self, *};
use super::IsToken::{*};

use super::expect_collection;

pub fn register_functs() -> Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)>
{
    let mut functions: Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)> = Vec::new();

    functions.push(("vector.magnitude".to_string(), magnitude));

    functions
}

pub fn register_vars() -> Vec<(std::string::String, Token)>
{
    let mut vars: Vec<(std::string::String, Token)> = Vec::new();

    vars.push(("vector3.one".to_string(), Collection(vec![Float(1.0), Float(1.0), Float(1.0)])));

    vars
}

fn magnitude(args: Vec<Token>) -> Option<Token>
{
    // if !expect(&args, vec![IsCollection]) { return None; }
    if !expect_collection(&args[0], vec![IsFloat, IsFloat, IsFloat]) {return None; }
    

    None
}
