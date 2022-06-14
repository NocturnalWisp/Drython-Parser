use std::collections::HashMap;

#[derive(Debug)]
pub struct Func
{
    pub name: String,
    pub size: usize,
    pub parameters: Vec<String>,
    pub expressions: ExpressionList
}

#[derive(Debug)]
pub struct ExpressionList
{
    pub size: usize,
    pub variables: HashMap<usize, (String, String)>,
    pub calls: HashMap<usize, (String, Vec<String>)>,
    pub internal_expressions: HashMap<usize, ExpressionList>,
    pub scope_expression: Option<String>
}

#[derive(Debug)]
pub struct RegisteredList<'a>
{
    pub func_list: Vec<&'a str>,
    pub var_list: Vec<&'a str>,
    pub error_list: Vec<String>
}