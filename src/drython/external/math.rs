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
    if !expect(&args, vec![IsCollection]) { return None; }
    if !expect_collection(&args[0], vec![IsFloat, IsFloat, IsFloat]) {return None; }
    

    None
}
