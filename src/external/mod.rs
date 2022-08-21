use crate::types::{Token, DynamicFunctionCall, RegisteredFunction, RegisteredVariable};

mod vector;
pub mod auto;
mod math;
mod collection;

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
    IsChar,
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
            (IsToken::IsChar, Token::Char(_)) => true,
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
            (IsToken::IsChar, Token::Char(_)) => false,
            (IsToken::IsCollection, Token::Collection(_)) => false,
            _ => false
        }
    }
}

pub fn get_lib(lib: &str) -> Result<(Vec<RegisteredFunction>, Vec<RegisteredVariable>), String>
{
    let mut functions: Vec<RegisteredFunction> = Vec::new();
    let mut vars: Vec<RegisteredVariable> = Vec::new();

    match lib
    {
        "vector" => { vector::register_functs(&mut functions); vector::register_vars(&mut vars); },
        "math" => { math::register_functs(&mut functions); math::register_vars(&mut vars); },
        "collection" => { collection::register_functs(&mut functions); collection::register_vars(&mut vars); },
        _ => { return Err(format!("No library found with the name: {}", lib)); }
    };

    Ok((functions, vars))
}

// These functions allow for extracting and expecting a specific token
// from the passed token arguments.

// Returns true if the discriminents match the arguments.
#[allow(dead_code)]
pub fn expect(args: &Vec<Token>, token_checks: &[Option<&[IsToken]>]) -> Result<(), String>
{
    if args.len() > token_checks.len()
    {
        return Err("Too many arguments were passed to this function.".to_string());
    }
    else if args.len() < token_checks.len()
    {
        return Err("Too few arguments passed for this function.".to_string());
    }
    
    for (i, check) in token_checks.iter().enumerate()
    {
        if let Some(check) = check
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
                return Err(format!("Unexpected argument type. Expected one of {:?} for argument {}.", check, i));
            }
        }
    }
    
    Ok(())
}

// Expects certain values within a collection.
// Use "expect()" before this for better error handling.
#[allow(dead_code)]
pub fn expect_collection(arg: &Token, argument_number: usize, token_checks: Vec<IsToken>) -> Result<(), String>
{
    if let Token::Collection(items) = arg
    {
        if items.len() != token_checks.len()
        {
            return Err(format!("Expected {} items in the collection for argument {}.", token_checks.len(), argument_number));
        }
        
        for (i, check) in token_checks.iter().enumerate()
        {
            if check != &items[i]
            {
                return Err(format!("Expected variable of type {:?} in collection indexed at {} for argument {}", check, i, argument_number));
            }
        }
    }
    else
    {
        return Err(format!("Expected a collection for argument {}", argument_number));
    }

    Ok(())
}

#[allow(dead_code)]
pub enum FunctionCall<T, U, R>
    where Token: From<R>
{
    A0(fn()),
    A0R(fn()->R), 

    A1(fn(T)),
    A1R(fn(T)->R),

    A2(fn(T, U)),
    A2R(fn(T, U)->R)
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
        FunctionCall::A0(call) =>
        {
            return Some(Box::new(move |_args| { call(); Ok(None) }));
        },
        FunctionCall::A0R(call) =>
        {
            return Some(Box::new(move |_args|
                    {
                        Ok(Some(Token::from(call())))
                    }));
        },
        FunctionCall::A1(call) =>
        {
            return Some(Box::new(move |args| { call(T::from(args[0].clone())); Ok(None) }));
        },
        FunctionCall::A1R(call) =>
        {
            return Some(Box::new(move |args|
                    {
                        Ok(Some(Token::from(call(T::from(args[0].clone())))))
                    }));
        },
        FunctionCall::A2(call) =>
        {
            return Some(Box::new(move |args| { call(T::from(args[0].clone()), U::from(args[1].clone())); Ok(None) }));
        },
        FunctionCall::A2R(call) =>
        {
            return Some(Box::new(move |args|
                    {
                        Ok(Some(Token::from(call(T::from(args[0].clone()), U::from(args[1].clone())))))
                    }));
        },
    }
}

#[macro_export]
macro_rules! register_function
{
    ($functions: expr, $name:expr, $call:expr) =>
    {
        $functions.push(($name.to_string(), attach::<i32, i32, i32>(FunctionCall::A0($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $t, $t>(FunctionCall::A1($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $u:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $u, $u>(FunctionCall::A2($call))));
    };
}

#[macro_export]
macro_rules! register_function_return
{
    ($functions: expr, $name:expr, $call:expr, $r:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$r, $r, $r>(FunctionCall::A0R($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $r:ty) =>
    {
        $functions.push(($name.to_string(), attach::<$t, $t, $r>(FunctionCall::A1R($call))));
    };
    ($functions: expr, $name:expr, $call:expr, $t:ty, $u:ty, $r:ty) =>
    {
        $functions.push((name.to_string(), attach::<$t, $u, $r>(FunctionCall::A2R($call))));
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

#[allow(unused_imports)]
pub (crate) use register_function;
pub (crate) use register_function_return;
pub (crate) use register_custom_function;
