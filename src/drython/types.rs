use std::{collections::HashMap};
use std::any::Any;

#[derive(Debug)]
pub struct ExpressionList
{
    pub scope_info: (Option<String>, Option<String>),
    pub size: usize,
    pub variables: HashMap<usize, (String, String)>,
    pub calls: HashMap<usize, (String, Vec<String>)>,
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
            variables: HashMap::new(),
            calls: HashMap::new(),
            internal_expressions: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub enum Token<'a>
{
    Int(i32),
    Float(f32),
    Bool(bool),
    Var(&'a str),
    Call(&'a str, &'a str),
    Operation(Box<Operation<'a>>)
}

#[derive(Debug)]
pub struct Operation<'a>
{
    pub op: char,
    pub next_op: Option<char>,
    pub a: Token<'a>,
    pub b: Token<'a>,
}

#[derive(Debug)]
pub struct RegisteredList<'a>
{
    pub func_list: Vec<&'a str>,
    pub var_list: Vec<&'a str>,
    pub error_list: Vec<String>
}