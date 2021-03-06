use std::mem::discriminant;

use crate::drython::types::Token::{self, *};
use super::IsToken::{*};

use super::{expect, expect_collection};

pub fn register_functs() -> Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)>
{
    let mut functions: Vec<(std::string::String, fn(Vec<Token>) -> Option<Token>)> = Vec::new();

    functions.push(("sqrt".to_string(), sqrt));

    functions
}

pub fn register_vars() -> Vec<(std::string::String, Token)>
{
    let mut vars: Vec<(std::string::String, Token)> = Vec::new();


    vars
}

fn sqrt(args: Vec<Token>) -> Option<Token>
{
    if !expect(&args, &[&[IsFloat, IsInt]]) { return None; }
    
    if let Token::Float(f) = args[0]
    {
        return Some(Token::Float(f.sqrt()));
    }
    else if let Token::Int(i) = args[0]
    {
        return Some(Token::Float((i as f32).sqrt()));
    }

    None
}
