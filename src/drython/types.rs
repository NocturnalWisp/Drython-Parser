use std::{collections::HashMap};

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
pub struct RegisteredList<'a>
{
    pub func_list: Vec<&'a str>,
    pub var_list: Vec<&'a str>,
    pub error_list: Vec<String>
}