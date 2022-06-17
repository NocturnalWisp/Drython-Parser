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
pub struct ExpressionList<'a>
{
    pub scope_info: (Option<String>, Option<String>),
    pub size: usize,
    pub variables: HashMap<usize, (String, VecDeque<Token<'a>>)>,
    pub calls: HashMap<usize, (String, VecDeque<Token<'a>>)>,
    pub internal_expressions: HashMap<usize, ExpressionList<'a>>,
}

impl<'a> ExpressionList<'a>
{
    pub fn new() -> ExpressionList<'a>
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