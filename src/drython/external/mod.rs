use std::{collections::HashMap, mem::{Discriminant, discriminant}};
use super::types::Token;

mod vector;

// Allows for quick checking if a token is of a certain type.
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
        self == &match other
        {
            Token::Null => IsToken::IsNull,
            Token::Int(_) => IsToken::IsInt,
            Token::Float(_) => IsToken::IsFloat,
            Token::Bool(_) => IsToken::IsBool,
            Token::String(_) => IsToken::IsString,
            Token::Collection(_) => IsToken::IsCollection,
            
            _ => IsToken::IsNone
        }
    }
}

pub fn get_libs(libs: Vec<&str>) -> Vec<(String, fn(Vec<Token>) -> Option<Token>)>
{
    let mut functions: Vec<(String, fn(Vec<Token>) -> Option<Token>)> = Vec::new();

    // Don't register multiple of the same lib.
    let mut registered: Vec<&str> = Vec::new();

    for lib in libs
    {
        if registered.contains(&lib) { continue; }

        let new_lib: Option<Vec<(String, fn(Vec<Token>) -> Option<Token>)>> = match lib
        {
            "vector" => Some(vector::register_functs()),
            _ => None
        };

        if let Some(result) = new_lib
        {
            for func in result
            {
                functions.push(func);
            }
        }

        registered.push(lib);
    }

    functions
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