use std::collections::VecDeque;
use std::{collections::HashMap};

macro_rules! skip_fail_operator
{
    ($res:expr) => {
        match $res {
            Token::Operator(val) => val,
            _ => {
                continue;
            }
        }
    };
}

#[derive(Debug)]
pub struct ExpressionList
{
    pub scope_info: (Option<String>, Option<String>),
    pub size: usize,
    pub single_op: HashMap<usize, (String, String)>,
    pub multi_ops: HashMap<usize, (String, Vec<String>)>,
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

#[derive(Debug)]
pub enum Token<'a>
{
    Int(i32),
    Float(f32),
    Bool(bool),
    Var(&'a str),
    Call(&'a str, &'a str),
    Operation(VecDeque<Token<'a>>),
    Operator(&'a str)
}

#[derive(Debug)]
pub struct Operation<'a>
{
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