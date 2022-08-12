use crate::drython::types::{Token, RegisteredFunction, RegisteredVariable};
use super::IsToken::{*};

use super::{expect, FunctionCall, attach, register_function, register_function_return};

pub fn register_functs() -> Vec<RegisteredFunction>
{
    let mut functions: Vec<RegisteredFunction> = Vec::new();

    // functions.push(("sqrt".to_string(), sqrt));
    functions.push(("sqrt".to_string(), attach::<f32, f32, f32>(FunctionCall::a1r(f32::sqrt))));
    register_function_return!(functions, "sqrt", f32::sqrt, f32, f32);

    functions
}

pub fn register_vars() -> Vec<RegisteredVariable>
{
    let mut vars: Vec<RegisteredVariable> = Vec::new();


    vars
}

// fn sqrt(args: Vec<Token>) -> Option<Token>
// {
//     if !expect(&args, &[&[IsFloat, IsInt]]) { return None; }
    
//     if let Token::Float(f) = args[0]
//     {
//         return Some(Token::Float(f.sqrt()));
//     }
//     else if let Token::Int(i) = args[0]
//     {
//         return Some(Token::Float((i as f32).sqrt()));
//     }

//     None
// }
