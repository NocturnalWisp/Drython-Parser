use std::{collections::HashMap, mem::{Discriminant, discriminant}};
use super::types::Token;

mod vector;
mod misc;
mod math;

// Allows for quick checking if a token is of a certain type.
#[derive(Debug)]
pub enum IsToken
{
    IsNull,
    IsInt,
    IsFloat,
    IsBool,
    IsString,
    IsCollection,
    IsNone
}

impl PartialEq<IsToken> for IsToken
{
    fn eq(&self, other: &IsToken) -> bool
    {
        self == other
    }
}

impl PartialEq<Token> for IsToken
{
    fn eq(&self, other: &Token) -> bool
    {
        match (self, other)
        {
            (IsToken::IsNull, Token::Null) => true,
            (IsToken::IsInt, Token::Int(_)) => true,
            (IsToken::IsFloat, Token::Float(_)) => true,
            (IsToken::IsBool, Token::Bool(_)) => true,
            (IsToken::IsString, Token::String(_)) => true,
            (IsToken::IsCollection, Token::Collection(_)) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Token) -> bool
    {
        match (self, other)
        {
            (IsToken::IsNull, Token::Null) => false,
            (IsToken::IsInt, Token::Int(_)) => false,
            (IsToken::IsFloat, Token::Float(_)) => false,
            (IsToken::IsBool, Token::Bool(_)) => false,
            (IsToken::IsString, Token::String(_)) => false,
            (IsToken::IsCollection, Token::Collection(_)) => false,
            _ => false
        }
    }
}

pub fn get_lib(lib: &str) -> (Vec<(String, fn(Vec<Token>) -> Option<Token>)>, Vec<(std::string::String, Token)>)
{
    let mut functions: Vec<(String, fn(Vec<Token>) -> Option<Token>)> = Vec::new();
    let mut vars: Vec<(std::string::String, Token)> = Vec::new();

    let result_option = match lib
    {
        "vector" => Some((vector::register_functs(), vector::register_vars())),
        "misc" => Some((misc::register_functs(), misc::register_vars())),
        "math" => Some((math::register_functs(), math::register_vars())),
        _ => None
    };

    if let Some(mut result) = result_option
    {
        functions.append(&mut result.0);
        vars.append(&mut result.1);
    }

    (functions, vars)
}

// These functions allow for extracting and expecting a specific token
// from the passed token arguments.

// Returns true if the discriminents match the arguments.
pub fn expect(args: &Vec<Token>, token_checks: Vec<IsToken>) -> bool
{
    if args.len() != token_checks.len()
    {
        return false
    }
    
    for (i, check) in token_checks.iter().enumerate()
    {
        if check != &args[i]
        {
            return false;
        }
    }
    
    return true;
}

// Expects certain values within a collection.
// Use "expect()" before this for better error handling.
pub fn expect_collection(arg: &Token, token_checks: Vec<IsToken>) -> bool
{
    if let Token::Collection(items) = arg
    {
        if items.len() != token_checks.len()
        {
            return false
        }
        
        for (i, check) in token_checks.iter().enumerate()
        {
            if check != &items[i]
            {
                return false;
            }
        }
    }
    else
    {
        return false;
    }

    return true;
}
