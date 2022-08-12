use super::types::{Token, DynamicFunctionCall, RegisteredFunction, RegisteredVariable};

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

pub fn get_lib(lib: &str) -> (Vec<RegisteredFunction>, Vec<RegisteredVariable>)
{
    let mut functions: Vec<RegisteredFunction> = Vec::new();
    let mut vars: Vec<RegisteredVariable> = Vec::new();

    match lib
    {
        "vector" => { vector::register_functs(&mut functions); vector::register_vars(&mut vars); },
        "math" => { math::register_functs(&mut functions); math::register_vars(&mut vars); },
        _ => ()
    };

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

#[allow(dead_code)]
pub enum FunctionCall<T, U, R>
    where Token: From<R>
{
    a0(fn()),
    a0r(fn()->R), 

    a1(fn(T)),
    a1r(fn(T)->R),

    a2(fn(T, U)),
    a2r(fn(T, U)->R)
}

pub fn attach<T, U, R>(function_call: FunctionCall<T, U, R>) -> DynamicFunctionCall
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

#[macro_export]
macro_rules! register_function
{
    ($functions: expr, $name:expr, $call:expr) =>
    {
        $functions.push(($name.to_string(), attach::<i32, i32, i32>(FunctionCall::a0($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $t, $t>(FunctionCall::a1($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $u:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $u, $u>(FunctionCall::a2($call))));
    };
}

#[macro_export]
macro_rules! register_function_return
{
    ($functions: expr, $name:expr, $call:expr, $r:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$r, $r, $r>(FunctionCall::a0r($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $r:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $t, $r>(FunctionCall::a1r($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $u:ty, $r:ty) =>
    {
        $functions.push((name.to_string(), attach::<$t, $u, $r>(FunctionCall::a2r($call))));
    };
}

#[macro_export]
macro_rules! register_custom_function
{
    ($functions: expr, $name:expr, $call:expr) =>
    {
        $functions.push(($name.to_string(), Some(Box::new($call))));
    }
}

pub (crate) use register_function;
pub (crate) use register_function_return;
pub (crate) use register_custom_function;
