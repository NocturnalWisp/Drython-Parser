use super::types::Token;

mod vector;
pub mod auto;
mod math;

// Allows for quick checking if a token is of a certain type.
#[derive(Debug)]
#[allow(dead_code)]
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

pub fn get_lib(lib: &str) -> (Vec<(String, Option<Box<dyn Fn(Vec<Token>) -> Option<Token>>>)>, Vec<(std::string::String, Token)>)
{
    let mut functions: Vec<(String, Option<Box<dyn Fn(Vec<Token>) -> Option<Token>>>)> = Vec::new();
    let mut vars: Vec<(std::string::String, Token)> = Vec::new();

    let result_option = match lib
    {
        "vector" => Some((vector::register_functs(), vector::register_vars())),
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
pub fn expect(args: &[Token], token_checks: &[&[IsToken]]) -> bool
{
    if args.len() != token_checks.len()
    {
        return false
    }
    
    for (i, check) in token_checks.iter().enumerate()
    {
        let mut has_token_type = false;
        for token in check.iter()
        {
            if token == &args[i]
            {
                has_token_type = true;
            }
        }

        if !has_token_type 
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

pub enum FunctionCall<T, U, R>
    where Token: From<R>
{
    a0(fn()),
    a0r(fn()->R), 

    a1(fn(T) -> R),
    a1r(fn(T)->R),

    a2(fn(T, U)),
    a2r(fn(T, U)->R)
}

pub fn attach<T, U, R>(function_call: FunctionCall<T, U, R>) -> Option<Box<dyn Fn(Vec<Token>) -> Option<Token>>>
    where
        Token: From<T>,
        T: 'static,
        T: From<Token>,

        Token: From<U>,
        U: 'static,
        U: From<Token>,
        
        Token: From<R>,
        R: 'static
{
    match function_call
    {
        FunctionCall::a0(call) =>
        {
            return Some(Box::new(move |args| { call(); return None; }));
        },
        FunctionCall::a0r(call) =>
        {
            return Some(Box::new(move |args|
                    {
                        Some(Token::from(call()))
                    }));
        },
        FunctionCall::a1(call) =>
        {
            return Some(Box::new(move |args| { call(T::from(args[0].clone())); return None; }));
        },
        FunctionCall::a1r(call) =>
        {
            return Some(Box::new(move |args|
                    {

                        println!("{:#?}", args);
                        Some(Token::from(call(T::from(args[0].clone()))))
                    }));
        },
        FunctionCall::a2(call) =>
        {
            return Some(Box::new(move |args| { call(T::from(args[0].clone()), U::from(args[1].clone())); return None; }));
        },
        FunctionCall::a2r(call) =>
        {
            return Some(Box::new(move |args|
                    {
                        Some(Token::from(call(T::from(args[0].clone()), U::from(args[1].clone()))))
                    }));
        },
        _ => { None }
    }
}
