use std::{collections::HashMap};
use arrayref;

use super::script_type::ScriptType;

#[derive(Debug)]
pub struct Parser
{
    pub script_type: ScriptType,
    pub global_expressions: ExpressionList
}

#[derive(Debug)]
pub struct ExpressionList
{
    pub scope_info: (Option<String>, Option<String>),
    pub size: usize,
    // Return, assignment
    pub single_op: HashMap<usize, (String, Vec<Token>)>,
    // Function call
    pub multi_ops: HashMap<usize, (String, Vec<Vec<Token>>)>,
    pub internal_expressions: HashMap<usize, ExpressionList>,
}

impl ExpressionList
{
    pub fn new() -> ExpressionList
    {
        ExpressionList
        {
            scope_info: (None, None),
            size: 0,
            single_op: HashMap::new(),
            multi_ops: HashMap::new(),
            internal_expressions: HashMap::new()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token
{
    Int(i32),
    Float(f32),
    Bool(bool),
    Var(String),
    Call(String, String),
    Operation(Vec<Token>),
    Operator(String)
}

#[derive(Debug)]
pub struct Operation
{
    pub a: Token,
    pub b: Token,
}

pub struct Runner<'a>
{
    pub parser: Parser,
    pub functions: HashMap<&'a str, fn(Vec<Token>)>,
    pub vars: HashMap<String, Token>
}

// Union of function pointer types for quick calling.
pub union FunctionPointer<R, T, U>
{
    f1: fn(),
    f2: fn(T),
    f3: fn(T, U),
    f4: fn() -> R,
    f5: fn(T) -> R,
    f6: fn(T, U) -> R
}